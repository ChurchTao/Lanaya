<template>
    <span class="text-sm absolute right-0.5 top-0.5">
        <div class="flex justify-end items-center">
            <div v-if="editable" v-for="tag, i in tags" class="badge badge-primary ml-1 gap-1" @click.stop="removeTag(i)">
                {{ tag }}
                <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" class="inline-block w-4 h-4 stroke-current">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12"></path>
                </svg>
            </div>
            <div v-else v-for="tag in tags" class="badge badge-primary ml-1">{{ tag }}</div>

            <div class="form-control ml-1" v-if="editable">
                <div class="input-group input-group-xs" @click.prevent.stop>
                    <input
                        type="text"
                        v-model="inputText"
                        :placeholder="$t('tags.placeholder')"
                        @keyup.enter.stop="addTags"
                        @keyup.esc.stop="$emit('onEscape')"
                        data-disable-hotkeys="true"
                        class="input input-xs input-primary input-bordered"
                        autofocus
                        autocomplete="off"
                        autoCorrect="off"
                        autoCapitalize="off"
                        spellCheck="false"
                    />
                    <button class="btn btn-primary btn-xs btn-square" @click="addTags">
                        <svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink"  class="h-4 w-4" viewBox="0 0 20 20">
                            <g fill="none"><path d="M10.5 2.75a.75.75 0 0 0-1.5 0V9H2.75a.75.75 0 0 0 0 1.5H9v6.25a.75.75 0 0 0 1.5 0V10.5h6.25a.75.75 0 0 0 0-1.5H10.5V2.75z" fill="currentColor"></path></g>
                        </svg>
                    </button>
                </div>
            </div>
        </div>
    </span>
</template>

<script setup>

import { ref } from "vue";

const emits = defineEmits(['onEscape'])
const props = defineProps({
    tags: Array,
    editable: Boolean
})

const inputText = ref("")

const removeTag = (tag) => {
    props.tags.splice(tag, 1)
}

const addTags = () => {
    const newTags = inputText.value.split(",").map(tag => tag.toLowerCase().trim())
    newTags.forEach(tag => {
        if (!!tag && !props.tags.includes(tag)) {
            props.tags.push(tag)
        }
    })
    inputText.value = ""
}

</script>

<style scoped>
.input {
    outline: none;

}
</style>