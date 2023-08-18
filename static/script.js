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
//enviar os dados do formulário como uma solicitação POST
async function sendResizeRotateRequest(data) {
    return fetch("/resize", {
        method: "POST",
        headers: {
            "Content-Type": "application/json",
        },
        body: JSON.stringify(data),
    }).then((response) => response.text());
}

async function sendRotateRequest(data) {
    return fetch("/rotate", {
        method: "POST",
        headers: {
            "Content-Type": "application/json",
        },
        body: JSON.stringify(data),
    }).then((response) => response.text());
}

async function sendDownloadRequest(filename) {
    return fetch(`/download/${filename}`, {
        method: "GET",
    }).then((response) => response.blob());
}

document.getElementById("resize-form").addEventListener("submit", async (event) => {
    event.preventDefault();

    // Ler os dados do formulário
    let { imageFile,  socialPlatformName } = readFormData();
    console.log(imageFile,  socialPlatformName);

    // Codificar o arquivo de imagem como uma string base64
    let imageData = await encodeImageFile(imageFile);

    let data = {
        input_data: imageData,
        social_platform_name: socialPlatformName,
    };


    
    // Enviar os dados do formulário como uma solicitação POST
    let text = await sendResizeRotateRequest(data);

    document.getElementById("result").textContent = text;
});

document.getElementById("rotate-form").addEventListener("submit", async (event) => {
    event.preventDefault();

    // Ler os dados do formulário
    let { imageFile, rotation } = readFormData();

    // Codificar o arquivo de imagem como uma string base64
    let imageData = await encodeImageFile(imageFile);

    let data = {
        input_data: imageData,
        rotation: rotation,
    };

    // Enviar os dados do formulário como uma solicitação POST
    let text = await sendRotateRequest(data);

    document.getElementById("result").textContent = text;
});

document.getElementById("download-form").addEventListener("submit", async (event) => {
    event.preventDefault();

    // Ler os dados do formulário
    let filename = document.getElementById("filename").value;

    // Enviar uma solicitação GET para a rota /download/{filename}
    let blob = await sendDownloadRequest(filename);

    // Criar um link para baixar o arquivo
    let url = URL.createObjectURL(blob);
    let a = document.createElement("a");
    a.href = url;
    a.download = filename;
    document.body.appendChild(a);
    a.click();
});