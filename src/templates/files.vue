
<template>
    <main>
        <div class="fileContainer" id="fileView">
            <div class="fileContainerResizer" id="fileViewResizer">

            </div>
        </div>
    </main>
</template>

<script>
    export default {
        mounted() {
            let resizer = document.getElementById('fileViewResizer')
            let fileView = document.getElementById("fileView")
            resizer.addEventListener("mouseover", (event) => {
                console.log("e")
                document.addEventListener("mousedown", (e) => {
                    console.log("a")
                    fileView.style.width = event.pageX - fileView.getBoundingClientRect().left + 'px';
                    console.log()
                })
            })
        }
    }
</script>
