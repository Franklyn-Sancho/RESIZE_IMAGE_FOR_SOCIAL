//ler os dados do formulário
function readFormData() {
    let imageFile = document.getElementById("input-file").files[0];
    let outputPathName = document.getElementById("output-path").value;
    let socialPlatformName = document.getElementById("social-platform").value;
    let rotation = document.getElementById("rotation").value;

    return { imageFile, outputPathName, socialPlatformName, rotation };
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
    return fetch("/resize_rotate", {
        method: "POST",
        headers: {
            "Content-Type": "application/json",
        },
        body: JSON.stringify(data),
    }).then((response) => response.text());
}

document.getElementById("resize-form").addEventListener("submit", async (event) => {
    event.preventDefault();

    // Ler os dados do formulário
    let { imageFile, outputPathName, socialPlatformName, rotation } = readFormData();

    // Codificar o arquivo de imagem como uma string base64
    let imageData = await encodeImageFile(imageFile);

    let data = {
        input_data: imageData,
        output_path_name: outputPathName,
        social_platform_name: socialPlatformName,
        rotation: rotation,
    };

    // Enviar os dados do formulário como uma solicitação POST
    let text = await sendResizeRotateRequest(data);

    document.getElementById("result").textContent = text;
});