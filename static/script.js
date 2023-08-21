function readFormData() {
    let imageFile = document.getElementById("input-file").files[0];
    let socialPlatformName = document.getElementById("social-platform").value;
    let rotation = document.getElementById("rotation").value;

    return { imageFile, socialPlatformName, rotation };
}

//converter o arquivo de imagem como uma string base64
function encodeImageFile(imageFile) {
    return new Promise((resolve) => {
        let reader = new FileReader();
        reader.readAsDataURL(imageFile);
        reader.onload = () => {
            let imageData = reader.result.split(",")[1];
            resolve(imageData);
        };
    });
}

async function sendRotateResizeRequest(data) {
    return fetch("/rotate-resize", {
        method: "POST",
        headers: {
            "Content-Type": "application/json",
        },
        body: JSON.stringify(data),
    });
}
async function sendDownloadRequest(filename) {
    return fetch(`/download/${filename}`, {
        method: "GET",
    }).then((response) => response.blob());
}

document.getElementById("rotate-resize-form").addEventListener("submit", async (event) => {
    event.preventDefault();

    // Ler os dados do formulário
    let { imageFile, socialPlatformName, rotation } = readFormData();

    // Codificar o arquivo de imagem como uma string base64
    let imageData = await encodeImageFile(imageFile);

    let data = {
        input_data: imageData,
        social_platform_name: socialPlatformName,
        rotation: rotation,
    };

   
    let response = await sendRotateResizeRequest(data);
    let blob = await response.blob();

    let url = URL.createObjectURL(blob);
    let a = document.createElement("a");
    a.href = url;
    a.download = "rotated_resized_image.jpg";
    document.body.appendChild(a);
    a.click();

    document.getElementById("result").textContent = "Imagem baixada com sucesso!";
});


