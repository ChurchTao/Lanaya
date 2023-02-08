use super::database::Record as DBRecord;
use crate::utils::dirs::app_data_dir;
use anyhow::Result;
use once_cell::sync::OnceCell;
use std::sync::Arc;
use tantivy::collector::TopDocs;
use tantivy::query::QueryParser;
use tantivy::schema::{
    Field, IndexRecordOption, TextFieldIndexing, TextOptions, INDEXED, STORED, TEXT,
};
use tantivy::tokenizer::NgramTokenizer;
use tantivy::{self, doc, schema::Schema, Index, TantivyError};
use tantivy::{Document, Snippet, SnippetGenerator, Term};

// 写入操作使用50MB内存，应该够用了
const DEFAULT_MEMORY_BYTES: usize = 50 * 1024 * 1024;

pub struct SearchHolder {
    searcher: Arc<Search>,
}

#[derive(Clone)]
pub struct Search {
    index: Index,
    #[allow(unused)]
    schema: Schema,
    query_parser: QueryParser,

    // fields
    id: Field,
    content: Field,
    create_time: Field,
}

#[derive(Debug, Clone)]
pub struct Record {
    pub id: u64,
    // 用于搜索时高亮显示使用
    pub content_highlight: Option<String>,
}

impl SearchHolder {
    pub fn global() -> &'static SearchHolder {
        static CONFIG: OnceCell<SearchHolder> = OnceCell::new();

        CONFIG.get_or_init(|| SearchHolder {
            searcher: Arc::new(Search::new()),
        })
    }

    pub fn searcher(&self) -> Arc<Search> {
        self.searcher.clone()
    }
}

impl Search {
    pub fn new() -> Self {
        let data_dir = app_data_dir().unwrap();
        let mut schema_builder = Schema::builder();
        let id = schema_builder.add_u64_field("id", INDEXED | STORED);
        let text_field_indexing = TextFieldIndexing::default()
            .set_tokenizer("ngram3")
            .set_index_option(IndexRecordOption::WithFreqsAndPositions);
        let text_options = TextOptions::default()
            .set_indexing_options(text_field_indexing)
            .set_stored();

        let content = schema_builder.add_text_field("content", text_options);
        let create_time = schema_builder.add_u64_field("create_time", INDEXED | STORED);
        let schema = schema_builder.build();
        let mut index = Index::open_in_dir(&data_dir).unwrap_or_else(|err| {
            if let TantivyError::OpenDirectoryError(_) | TantivyError::OpenReadError(_) = err {
                std::fs::create_dir_all(&data_dir).expect("create index directory");
                Index::create_in_dir(&data_dir, schema.clone()).unwrap()
            } else {
                panic!("Error opening index: {err:?}")
            }
        });
        // let settings = index.settings_mut();
        // settings.sort_by_field = Some(IndexSortByField {
        //     field: "create_time".to_owned(),
        //     order: Order::Desc,
        // });
        index
            .tokenizers()
            .register("ngram3", NgramTokenizer::new(3, 3, false));
        let _ = index.set_default_multithread_executor();
        let query_parser = QueryParser::for_index(&index, vec![content]);

        Search {
            index,
            schema,
            query_parser,
            id,
            content,
            create_time,
        }
    }

    pub fn add_record(&self, record: &DBRecord) -> Result<()> {
        let mut writer = self.index.writer(DEFAULT_MEMORY_BYTES)?;
        writer.add_document(doc!(
            self.id => record.id,
            self.content => record.content.as_str(),
            self.create_time => record.create_time,
        ))?;
        writer.commit()?;
        Ok(())
    }

    pub fn search(&self, query: &str, limit: usize) -> Result<Vec<Record>> {
        let reader = self.index.reader()?;
        let searcher = reader.searcher();
        let query = self.query_parser.parse_query(query)?;
        let top_docs = searcher.search(&query, &TopDocs::with_limit(limit))?;
        // 匹配到content的话，高亮返回
        let snippet_generator = SnippetGenerator::create(&searcher, &*query, self.content)?;
        let mut records = vec![];
        for (_score, doc_address) in top_docs {
            let doc = searcher.doc(doc_address)?;
            let content = snippet_generator.snippet_from_doc(&doc);
            let record = self.format_record_res(doc, Some(content));
            records.push(record);
        }
        Ok(records)
    }

    pub fn clear_all(&self) -> Result<()> {
        let mut writer = self.index.writer(DEFAULT_MEMORY_BYTES)?;
        let _ = writer.delete_all_documents();
        writer.commit()?;
        Ok(())
    }

    #[allow(unused)]
    pub fn update_by_id(&self, id: u64, record: &DBRecord) -> Result<()> {
        let mut writer = self.index.writer(DEFAULT_MEMORY_BYTES)?;
        writer.delete_term(Term::from_field_u64(self.id, id));
        writer.add_document(doc!(
            self.id => id,
            self.content => record.content.as_str(),
            self.create_time => record.create_time,
        ))?;
        writer.commit()?;
        Ok(())
    }

    pub fn rebuild_from_db(&self, all_record: Vec<DBRecord>) -> Result<()> {
        let mut writer = self.index.writer(DEFAULT_MEMORY_BYTES)?;
        writer.delete_all_documents()?;
        for record in all_record {
            writer.add_document(doc!(
                self.id => record.id,
                self.content => record.content.as_str(),
                self.create_time => record.create_time,
            ))?;
            println!("add record: {:?}", record)
        }
        writer.commit()?;
        Ok(())
    }

    fn format_record_res(&self, doc: Document, snippet: Option<Snippet>) -> Record {
        Record {
            id: doc.get_first(self.id).unwrap().as_u64().unwrap(),
            content_highlight: snippet.map(|s| s.to_html()),
        }
    }
}

#[test]
fn test_search() {
    let search = SearchHolder::global().searcher();

    // let all = SqliteDB::new().find_all().unwrap();

    // let _ = search.rebuild_from_db(all);

    // let add_record = DBRecord {
    //     id: 1,
    //     content: "hello world".to_string(),
    //     md5: "".to_string(),
    //     create_time: 1,
    //     is_favorite: false,
    // };
    //
    // search.add_record(&add_record).unwrap();

    let res = search.search("r", 100);

    println!("{:?}", res);
}
