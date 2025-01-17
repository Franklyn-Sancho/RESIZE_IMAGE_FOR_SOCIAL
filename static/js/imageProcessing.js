import { readFormData } from './readFormData.js';
import { updatePreviewImage } from './imagePreview.js';
import { encodeImageFile } from './encodeImageFile.js';

async function sendAdjustment(type, value) {
    const { imageFile } = readFormData();
    const imageData = await encodeImageFile(imageFile);

    const data = {
        input_data: imageData,
    };

    console.log('Dados preparados:', data);

    // Adiciona o ajuste específico ao payload
    data[type] = value;

    try {
        const response = await fetch('/adjust-image', {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
            },
            body: JSON.stringify(data),
        });
    
        if (!response.ok) {
            const errorMessage = await response.text();
            throw new Error(`Erro do servidor: ${errorMessage}`);
        }
    
        const result = await response.json();
        const processedImageUrl = result.filename;
    
        const imageResponse = await fetch(`/download/${processedImageUrl}`);
        const blob = await imageResponse.blob();
        const newImage = URL.createObjectURL(blob);
        updatePreviewImage(newImage);
    
        console.log(`${type} ajustado com valor: ${value}`);
    } catch (error) {
        console.error(`Erro ao ajustar ${type}:`, error);
    }
}

// Eventos para brilho, contraste e escala de cinza
document.getElementById('brightness-container').addEventListener('input', function (e) {
    const brightness = parseInt(e.target.value, 10);
    sendAdjustment('brightness', brightness);
});

document.getElementById('contrast-container').addEventListener('input', function (e) {
    const contrast = parseInt(e.target.value, 10);
    sendAdjustment('contrast', contrast);
});

document.getElementById('greyscale-container').addEventListener('change', function (e) {
    const greyscale = e.target.checked;
    sendAdjustment('greyscale', greyscale);
});


document.getElementById('social-platform').addEventListener('change', async function () {
    try {
        const { imageFile, socialPlatformName } = readFormData();

        const imageData = await encodeImageFile(imageFile);

        const data = {
            input_data: imageData,
            social_platform_name: socialPlatformName,
        };

        console.log('Dados preparados:', data);

        // Envia os dados para o servidor
        fetch('/resize-image', {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
            },
            body: JSON.stringify(data),
        })
            .then(response => response.json())
            .then(result => {
                const processedImageUrl = result.filename; // Nome do arquivo retornado pelo servidor
                console.log('Nome do arquivo processado:', processedImageUrl);

                // Atualizar a visualização com a nova imagem processada
                fetch(`/download/${processedImageUrl}`)
                    .then(response => response.blob())
                    .then(blob => {
                        const newImage = URL.createObjectURL(blob);
                        updatePreviewImage(newImage);
                    });
            })
            .catch(error => console.error('Erro ao redimensionar imagem:', error));
    } catch (error) {
        console.error('Erro ao preparar os dados:', error);
    }
},

    document.getElementById('rotation').addEventListener('change', async function () {
        try {
            const { imageFile, rotation } = readFormData();

            const imageData = await encodeImageFile(imageFile);

            const data = {
                input_data: imageData,
                rotation: rotation,
            };

            console.log('Dados preparados:', data);

            // Envia os dados para o servidor
            fetch('/rotate-image', {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json',
                },
                body: JSON.stringify(data),
            })
                .then(response => response.json())
                .then(result => {
                    const processedImageUrl = result.filename; // Nome do arquivo retornado pelo servidor
                    console.log('Nome do arquivo processado:', processedImageUrl);

                    // Atualizar a visualização com a nova imagem processada
                    fetch(`/download/${processedImageUrl}`)
                        .then(response => response.blob())
                        .then(blob => {
                            const newImage = URL.createObjectURL(blob);
                            updatePreviewImage(newImage);
                        });
                })
                .catch(error => console.error('Erro ao girar imagem:', error));
        } catch (error) {
            console.error('Erro ao preparar os dados:', error);
        }
    }),

    document.getElementById('conversion-format').addEventListener('change', async function () {

        const { imageFile, conversionFormat } = readFormData();

        const imageData = await encodeImageFile(imageFile);

        const data = {
            input_data: imageData,
            format: conversionFormat,
        };

        console.log('Dados preparados:', data);

        // Envia os dados para o servidor
        fetch('/convert-image', {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
            },
            body: JSON.stringify(data),
        })
            .then(response => response.json())
            .then(result => {
                const processedImageUrl = result.filename; // Nome do arquivo retornado pelo servidor
                console.log('Nome do arquivo processado:', processedImageUrl);

                // Atualizar a visualização com a nova imagem processada
                fetch(`/download/${processedImageUrl}`)
                    .then(response => response.blob())
                    .then(blob => {
                        const newImage = URL.createObjectURL(blob);
                        updatePreviewImage(newImage);
                    });
            })
            .catch(error => console.error('Erro ao converter imagem:', error));
    }),

    document.getElementById('download-button').addEventListener('click', function (event) {
        event.preventDefault();
        const url = `/download/${processedImageUrl}`; // Certifique-se de que processedImageUrl esteja definido corretamente
        window.location.href = url; // Baixar a imagem processada
    }));