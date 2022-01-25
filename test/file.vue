<template>
    <template>
        <template></template>
        <template></template>
    </template>
    <template></template>
    <template></template>
</template>

<script lang="ts" setup>
import { ExclamationIcon } from '@heroicons/vue/solid';
import { Dialog, DialogOverlay, DialogTitle, TransitionChild, TransitionRoot } from '@headlessui/vue';
import { watch } from "vue";

import type { NewQuoteRequest, QuoteRequest, QuoteRequestId, QuoteRequestPatch } from "@/resources/quoteRequests";
import type { ApiError, AbortError, NetworkError } from "@/error";
import { usePassport } from "@/passport";
import { createQuoteRequest, retrieveQuoteRequest, updateQuoteRequest, destroyQuoteRequest } from "@/resources/quoteRequests";

</script>
