<template>
    <div class="taskbarContainer">
        <div class="taskbarContainerItems">
            <div>
                <div class="taskbarContainerItem">File</div>
                <div class="taskbarDropDownMenu">
                    <div class="taskbarDropDownMenuItem" onclick="">Open Folder</div>
                    <div class="taskbarDropDownMenuItem" onclick="">Open File</div>

                </div>
            </div>
            <div>
                <div class="taskbarContainerItem">Edit</div>
                <div class="taskbarDropDownMenu">
                </div>
            </div>            
            <div>
                <div class="taskbarContainerItem">View</div>
                <div class="taskbarDropDownMenu">
                    
                </div>
            </div>
        </div>
    </div>
</template>

<script>
import { invoke } from '@tauri-apps/api/core';

    export default {
        mounted() {
            let taskBarItems = document.getElementsByClassName('taskbarContainerItem')
            let taskBarAllItems = document.querySelector('.taskbarContainerItems')

            document.addEventListener("click", (event) => {
                let children = taskBarAllItems.childNodes
                for (const elem of children) {
                    let dropdown = elem.querySelector(".taskbarDropDownMenu")
                    if (!elem.contains(event.target)) {
                        dropdown.style.display = 'none'
                    } else {
                        dropdown.style.display = 'block'
                    }
                }
            });
        }
    }
</script>