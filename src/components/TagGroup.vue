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
                        placeholder="Search…"
                        @keyup.enter.stop="addTags"
                        @keyup.esc.stop="$emit('onEscape')"
                        data-disable-hotkeys="true"
                        class="input input-xs input-primary input-bordered"
                    />
                    <button class="btn btn-primary btn-xs btn-square" @click="addTags">
                        <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z" />
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