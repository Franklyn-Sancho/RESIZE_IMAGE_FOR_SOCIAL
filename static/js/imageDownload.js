import { prepareRequestData } from './prepareRequestData.js';

// Função para redimensionar a imagem
async function resizeImage(imageData, socialPlatformName) {
  const requestPayload = {
    input_data: imageData,  // A imagem em base64 ou binária
    social_platform_name: socialPlatformName,
  };

  const response = await fetch("/resize-image", {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify(requestPayload),
  });

  const result = await response.json();
  console.log("Arquivo redimensionado:", result);
  return result;  // Retorna o nome do arquivo ou dados da resposta
}

async function readImageAsBase64(file) {
  return new Promise((resolve, reject) => {
    if (!(file instanceof File)) {
      reject(new TypeError("O argumento fornecido não é um arquivo válido."));
      return;
    }

    const reader = new FileReader();
    reader.onload = () => resolve(reader.result.split(",")[1]); // Remove o prefixo base64
    reader.onerror = (error) => reject(error);
    reader.readAsDataURL(file); // Lê o arquivo
  });
}

async function rotateImage(file, rotation) {
  if (!file) {
    console.error("Nenhum arquivo foi fornecido para rotação.");
    return;
  }

  try {
    const base64Image = await readImageAsBase64(file);
    const requestPayload = {
      input_data: base64Image, // Conteúdo base64 da imagem
      rotation: rotation,      // "None", "Right90", "Left90", "HalfCircle"
    };

    console.log("Payload enviado para /rotate-image:", requestPayload);

    const response = await fetch("http://localhost:8080/rotate-image", {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify(requestPayload),
    });

    if (!response.ok) {
      const errorText = await response.text();
      console.error("Erro no servidor:", errorText);
      throw new Error(`Erro ao rotacionar imagem: ${response.statusText}`);
    }

    const result = await response.json();
    console.log("Imagem rotacionada com sucesso:", result);
    return result;
  } catch (error) {
    console.error(error.message);
  }
}

// Função para ajustar a imagem (brilho, contraste, escala de cinza)
async function adjustImage(imageData, brightness, contrast, greyscale) {
  const requestPayload = {
    input_data: imageData,  // A imagem em base64 ou binária
    brightness: brightness,  // Valor de brilho
    contrast: contrast,      // Valor de contraste
    greyscale: greyscale,    // true/false para escala de cinza
  };

  const response = await fetch("/adjust-image", {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify(requestPayload),
  });

  const result = await response.json();
  console.log("Imagem ajustada:", result);
  return result;
}

// Função para converter a imagem
async function convertImage(imageData, format) {
  const requestPayload = {
    input_data: imageData,  // A imagem em base64 ou binária
    format: format,  // Formato de conversão (ex: "jpg", "png")
  };

  const response = await fetch("/convert-image", {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify(requestPayload),
  });

  const result = await response.json();
  console.log("Imagem convertida:", result);
  return result;
}

// Função para download da imagem
async function downloadImage(processedImage, format) {
  // Cria um Blob a partir da imagem processada
  const blob = await processedImage.blob(); // Verifique se `processedImage` contém a resposta de imagem em Blob
  const href = URL.createObjectURL(blob);
  const anchor = document.createElement("a");
  anchor.href = href;

  // Definindo a extensão do arquivo
  const extension = format === "png" ? ".png" : ".jpg";
  const filename = `processed_image${extension}`;

  anchor.download = filename;
  document.body.appendChild(anchor);
  anchor.click();
  document.body.removeChild(anchor); // Limpa o DOM após o clique
}

document
  .getElementById("image-form")
  .addEventListener("submit", async (event) => {
    event.preventDefault();

    const downloadButton = document.getElementById("download-button");
    const resultDiv = document.getElementById("result");

    downloadButton.classList.add("loading");
    downloadButton.disabled = true;

    try {
      resultDiv.textContent = "processing...";
      let data = await prepareRequestData();

      const imageData = data.input_data;
      let processedImage = imageData;

      // Redimensionar a imagem
      processedImage = await resizeImage(processedImage, data.social_platform_name);

      // Rotacionar a imagem
      if (data.rotation) {
        processedImage = await rotateImage(processedImage, data.rotation);
      }

      // Ajustar a imagem
      processedImage = await adjustImage(processedImage, data.brightness, data.contrast, data.greyscale);

      // Converter a imagem
      processedImage = await convertImage(processedImage, data.format);

      // Baixar a imagem final
      await downloadImage(processedImage, data.format);

      resultDiv.textContent = "successfully downloaded image";
    } catch (error) {
      console.error(error);
      resultDiv.textContent = error.message;
    } finally {
      downloadButton.classList.remove("loading");
      downloadButton.disabled = false;
    }
  });

