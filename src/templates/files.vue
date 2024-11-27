<template>
    <main>
        <div class="fileContainer" id="fileView">
            <div class="fileContainerHeader">
                <div class="fileContainerHeaderProjectName">PROJECT NAME</div>
                <div class="fileContainerHeaderFileTools">
                    <div class="fileContainerHeaderFileToolsTool">
                        <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" fill="currentColor" class="fileContainerHeaderFileToolsToolImg" viewBox="0 0 16 16">
                            <path d="M8 6.5a.5.5 0 0 1 .5.5v1.5H10a.5.5 0 0 1 0 1H8.5V11a.5.5 0 0 1-1 0V9.5H6a.5.5 0 0 1 0-1h1.5V7a.5.5 0 0 1 .5-.5"/>
                            <path d="M14 4.5V14a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V2a2 2 0 0 1 2-2h5.5zm-3 0A1.5 1.5 0 0 1 9.5 3V1H4a1 1 0 0 0-1 1v12a1 1 0 0 0 1 1h8a1 1 0 0 0 1-1V4.5z"/>
                        </svg>
                    </div>
                    <div class="fileContainerHeaderFileToolsTool">
                        <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" fill="currentColor" class="fileContainerHeaderFileToolsToolImg" viewBox="0 0 16 16">
                            <path d="m.5 3 .04.87a2 2 0 0 0-.342 1.311l.637 7A2 2 0 0 0 2.826 14H9v-1H2.826a1 1 0 0 1-.995-.91l-.637-7A1 1 0 0 1 2.19 4h11.62a1 1 0 0 1 .996 1.09L14.54 8h1.005l.256-2.819A2 2 0 0 0 13.81 3H9.828a2 2 0 0 1-1.414-.586l-.828-.828A2 2 0 0 0 6.172 1H2.5a2 2 0 0 0-2 2m5.672-1a1 1 0 0 1 .707.293L7.586 3H2.19q-.362.002-.683.12L1.5 2.98a1 1 0 0 1 1-.98z"/>
                            <path d="M13.5 9a.5.5 0 0 1 .5.5V11h1.5a.5.5 0 1 1 0 1H14v1.5a.5.5 0 1 1-1 0V12h-1.5a.5.5 0 0 1 0-1H13V9.5a.5.5 0 0 1 .5-.5"/>
                        </svg>
                    </div>
                </div>
            </div>
            <div class="fileContainerResizer" id="fileViewResizer" />

        </div>
    </main>
</template>

<script>
    export default {
        mounted() {
            let resizer = document.getElementById('fileViewResizer')
            let fileView = document.getElementById("fileView")
            
            resizer.addEventListener("mousedown", (event) => {
                document.addEventListener("mousemove", resize)
                document.addEventListener("mouseup", stopResize)
            })

            function resize(event) {
                fileView.style.width = event.pageX - fileView.getBoundingClientRect().left + 10 + 'px';
                document.body.style.cursor = 'col-resize';
                fileView.style.outlineColor = '#ff5afc'
            }

            function stopResize() {
                document.removeEventListener("mousemove", resize);
                document.removeEventListener("mouseup", stopResize);
                document.body.style.cursor = 'default';
                
            }

            resizer.addEventListener("mouseover", (event) => fileView.style.outlineColor = '#ff5afc');
            resizer.addEventListener("mouseleave", (event) => fileView.style.outlineColor = '#1f1f1f')
        }
    }
</script>