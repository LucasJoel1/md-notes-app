<template>
    <div class="editorContainer" id="editorContainer">
        <div id="editorLines" class="editorLines">
            <div class="editorLines-line">1</div>
        </div>

        <div class="editor" id="editor">
            <div id="cursor" class="editor-cursor">|</div>
        </div>
    </div>
</template>

<script>
    import { ref, reactive } from 'vue';
    import { invoke } from '@tauri-apps/api/core';
    import { location } from '../assets/js/state';

    export default {
        mounted() {
            let editor = document.getElementById("editor");
            let lines = document.getElementById("editorLines");
            let cursor = document.getElementById("cursor");
            editor.style.height = lines.childNodes.length * 20 + 'px'
            let rawText = [];

            function insertNewLine() {
                let index = lines.childNodes.length
                let newElem = document.createElement("div");
                newElem.classList = "editorLines-line"
                newElem.innerHTML = parseInt(index) + 1
                lines.appendChild(newElem)
                editor.style.height = lines.childNodes.length * 20 + 'px'
                newElem.scrollIntoView({ behavior: 'smooth', block: 'end' });
                location.char = 0;
                location.line = location.line++; 
                setProperCursorPos();
                rawText[location.line + 1] = new String();
            }

            function removePreviousLine() {
                if (lines.childNodes.length > 0 && location.char <= 0) {
                    console.log(location.line)
                    rawText[location.line] += rawText[location.line + 1]
                    rawText.splice(location.line - 1, 1);
                    lines.removeChild(lines.lastChild)
                    editor.style.height = lines.childNodes.length * 20 + 'px'       
                }
            }

            function setProperCursorPos() {
                cursor.style.left = (location.char * 9.609375) - (9.609375 / 2) + 'px'
                cursor.style.top = (location.line * 20) - 2 + 'px'
            }

            function moveCursorRight(_amount = 1) {
                location.char += _amount;
                cursor.style.left = (location.char * 9.609375) - (9.609375 / 2) + 'px'
            }

            function moveCursorLeft(_amount = 1) {
                if (location.char > 0) {
                    location.char -= _amount;
                    cursor.style.left = (location.char * 9.609375) - (9.609375 / 2) + 'px'
                }
            }

            function moveCursorDown(_amount = 1) {
                if (location.line + _amount > rawText.length) return;
                location.line += _amount;
                cursor.style.top = (location.line * 20) - 2 + 'px'
            }

            function moveCursorUp(_amount = 1) {
                if (location.line > 0) {
                    location.line -= _amount
                    cursor.style.top = (location.line * 20) - 2 + 'px'
                }

            }

            document.addEventListener("keydown", async (e) => {
                switch (e.key) {
                    case "Enter":
                        insertNewLine();
                        moveCursorDown();
                        break;
                    case "Backspace":
                        if (location.char == 0 && location.line > 0) {
                            removePreviousLine();
                            moveCursorUp();
                            break;
                        }
                        moveCursorLeft();
                        rawText[location.line] = await invoke("replace_char", {str: rawText[location.line], index: location.char, char: '\0'});
                        break;
                    case "ArrowRight":
                        moveCursorRight();
                        break;
                    case "ArrowLeft":
                        moveCursorLeft();
                        break;
                    case "ArrowDown":
                        moveCursorDown();
                        break;
                    case "ArrowUp":
                        moveCursorUp();
                        break;
                    default:
                        if (rawText[location.line] == undefined) {
                            rawText[location.line] = new String();
                        }
                        if (e.key.length == 1) {
                            rawText[location.line] = await invoke("replace_char", {str: rawText[location.line], index: location.char, char: e.key});
                            moveCursorRight();
                        }
                        break;
                }
                updateEditor();

                console.log(rawText)
            })

            function updateEditor() {
                editor.textContent = ''
                rawText.forEach((line) => {
                    const lineElem = document.createElement("div");
                    lineElem.classList.add("editor-text-syntax");
                    lineElem.textContent = line;
                    editor.appendChild(lineElem);
                    editor.appendChild(document.createElement('br'))
                });
                editor.style.height = lines.childNodes.length * 20 + 'px';
            }

            window.insertNewLine = insertNewLine;
            window.removePreviousLine = removePreviousLine;

            window.addEventListener("keydown", function(e) {
                if(["Space","ArrowUp","ArrowDown","ArrowLeft","ArrowRight"].indexOf(e.code) > -1) {
                    e.preventDefault();
                }
            }, false);
        }
    }
</script>