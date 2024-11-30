<template>
    <div class="infobarContainer">
        <div class="infobarItem-right infobarItem" id="mode">-- INSERT --</div>
        <div class="infobarItem-right infobarItem" id="input">ctrl + g + 19</div>
        <div class="infobarItem-left infobarItem" id="pos">Ln {{ location.line + 1 }}, Col {{ location.char + 1 }}</div>
    </div>
</template>

<script setup>
import { location } from '../assets/js/state'
</script>