document.getElementById("resize-form").addEventListener("submit", (event) => {
    event.preventDefault();

    // Ler os dados do formulário
    let imageFile = document.getElementById("input-file").files[0];
    console.log(`image file: ${imageFile}`)
    let outputPathName = document.getElementById("output-path").value;
    let socialPlatformName = document.getElementById("social-platform").value;


    // Codificar o arquivo de imagem como uma string base64
    let reader = new FileReader();
    reader.readAsDataURL(imageFile);
    reader.onload = () => {
        console.log('Antes da definição de imageData');
        let imageData = reader.result.split(",")[1];
        console.log('Depois da definição de imageData');

        let data = {
            input_data: imageData,
            output_path_name: outputPathName,
            social_platform_name: socialPlatformName,
        };
        console.log(data);

        // Enviar os dados do formulário como uma solicitação POST com o corpo no formato application/json
        fetch("/resize", {
            method: "POST",
            headers: {
                "Content-Type": "application/json",
            },
            body: JSON.stringify(data),
        })
            .then((response) => response.text())
            .then((text) => {
                document.getElementById("result").textContent = text;
            });
    };
});
