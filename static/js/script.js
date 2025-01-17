import { prepareRequestData } from './prepareRequestData.js';


let processedImageUrl = "";

//readformdata.js

export function readFormData() {
    const imageFile = document.getElementById("input-file").files[0];
    const socialPlatformName = document.getElementById("social-platform").value;
    const rotation = document.getElementById("rotation").value;
    const conversionFormat = document.getElementById("conversion-format").value;
    const brightness = document.getElementById("brightness").value;
    const contrast = document.getElementById("contrast").value;
    const greyscale = document.getElementById("greyscale").checked;

    updatePreview(imageFile, rotation, socialPlatformName);

    return { imageFile, socialPlatformName, rotation, conversionFormat, brightness, contrast, greyscale };
}

//encodeimagefile.js

export async function encodeImageFile(imageFile) {
    return new Promise((resolve, reject) => {

        if (!imageFile) {
            reject(new Error("No image file selected"));
            return;
        }

        const reader = new FileReader();
        reader.readAsDataURL(imageFile);
        reader.onload = () => {
            const imageData = reader.result.split(",")[1];
            resolve(imageData);
        };
    });
}

//updatepreview.js

const inputFile = document.querySelector("#input-file");
inputFile.addEventListener("change", updatePreviewImage);

const rotationSelect = document.querySelector("#rotation");
rotationSelect.addEventListener("change", updatePreviewImage);

const socialPlatformSelect = document.querySelector("#social-platform");
socialPlatformSelect.addEventListener("change", updatePreviewImage);

export function updatePreview(imageFile, rotation, socialPlatformName) {

    if (!imageFile) {
        console.error("No image file selected");
        return;
    }

    const objectURL = URL.createObjectURL(imageFile);
    const previewImage = document.querySelector("#preview-image");
    previewImage.src = objectURL;
    applyRotation(previewImage, rotation);
    applyResizing(previewImage, socialPlatformName);
}

function applyRotation(imageElement, rotation) {

    const rotationMap = {
        Right90: "rotate(90deg)",
        Left90: "rotate(-90deg)",
        HalfCircle: "rotate(180deg)",
        None: "none",
    };


    const transformValue = rotationMap[rotation] || "none";


    imageElement.style.transform = transformValue;
}

function updatePreviewImage() {
    // Get selected file, social platform name and rotation
    const file = document.querySelector("#input-file").files[0];
    const socialPlatformName = document.querySelector("#social-platform").value;
    const rotation = document.querySelector("#rotation").value;

    // Update preview image
    updatePreview(file, rotation, socialPlatformName);
}


//FUNCTIONS TO APPLY RESIZE ON IMAGES 
function applyResizing(imageElement, socialPlatformName) {
    // Define social platform dimensions map
    const socialPlatformDimensionsMap = {
        Facebook: { width: 1200, height: 630 },
        Instagram: { width: 1080, height: 1080 },
        Twitter: { width: 1200, height: 675 },
        Linkedin: { width: 1200, height: 627 },
    };

    // Get dimensions from social platform dimensions map
    const dimensions =
        socialPlatformDimensionsMap[socialPlatformName] || { width: "auto", height: "auto" };

    // Calculate scaled dimensions
    const [scaledWidth, scaledHeight] = calculateScaledDimensions(dimensions);

    // Apply dimensions to image element
    imageElement.style.width = `${scaledWidth}px`;
    imageElement.style.height = `${scaledHeight}px`;
}

function calculateScaledDimensions(dimensions) {
    // Calculate aspect ratio
    const aspectRatio = dimensions.width / dimensions.height;

    // Calculate scaled dimensions
    let scaledWidth, scaledHeight;
    if (dimensions.width > dimensions.height) {
        scaledWidth = Math.min(dimensions.width, 500);
        scaledHeight = scaledWidth / aspectRatio;
    } else {
        scaledHeight = Math.min(dimensions.height, 500);
        scaledWidth = scaledHeight * aspectRatio;
    }

    return [scaledWidth, scaledHeight];
}

//FUNCTIONS TO APPLY RESIZE ON IMAGES
//imageProcessing.js

// Função para ajustar a imagem
// Função genérica para enviar ajustes ao backend
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

//adjustimage.js

let brightnessButton = document.querySelector("#brightness-button");
let brightnessContainer = document.querySelector("#brightness-container");
let contrastButton = document.querySelector("#contrast-button");
let contrastContainer = document.querySelector("#contrast-container");
let greyscaleButton = document.querySelector("#greyscale-button");
let greyscaleContainer = document.querySelector("#greyscale-container");

brightnessButton.addEventListener("click", () => {
    toggleContainer(brightnessContainer);
});

contrastButton.addEventListener("click", () => {
    toggleContainer(contrastContainer);
});

greyscaleButton.addEventListener("click", () => {
    toggleContainer(greyscaleContainer);
});

document.addEventListener("DOMContentLoaded", () => {
    const brightnessInput = document.querySelector("#brightness");
    const brightnessOutput = document.querySelector("output[for=brightness]");
    updateOutput(brightnessInput, brightnessOutput);

    const contrastInput = document.querySelector("#contrast");
    const contrastOutput = document.querySelector("output[for=contrast]");
    updateOutput(contrastInput, contrastOutput);
});

function toggleContainer(container) {
    if (container.style.display === "none") {
        container.style.display = "block";
    } else {
        container.style.display = "none";
    }
}

function updateOutput(input, output) {
    input.addEventListener("input", () => {
        output.textContent = input.value;
    });
}
