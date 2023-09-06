function readFormData() {
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

async function encodeImageFile(imageFile) {
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

function updatePreview(imageFile, rotation, socialPlatformName) {

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

async function prepareRequestData() {
  const { imageFile, socialPlatformName, rotation, conversionFormat, brightness, contrast, greyscale } = readFormData();
  const imageData = await encodeImageFile(imageFile);

  return {
    input_data: imageData,
    social_platform_name: socialPlatformName,
    rotation: rotation,
    format: conversionFormat,
    brightness: Number(brightness),
    contrast: Number(contrast),
    greyscale: greyscale,
  };
}
async function downloadImage(data) {
  const response = await sendRotateResizeRequest(data);
  const blob = await response.blob();

  const href = URL.createObjectURL(blob);
  const anchor = document.createElement("a");
  anchor.href = href;

  const extension = {
    jpeg: ".jpeg",
    png: ".png",
  }[data.format] || ".jpg";
  const filename = `rotated_resized_image${extension}`;

  anchor.download = filename;
  document.body.appendChild(anchor);
  anchor.click();
}


document
  .getElementById("image-form")
  .addEventListener("submit", async (event) => {
    event.preventDefault();

    try {
      // Ler os dados do formulário
      let data = await prepareRequestData();

      await downloadImage(data);

      document.getElementById("result").textContent =
        "successfully downloaded image";
    } catch (error) {
      console.error(error);
      document.getElementById("result").textContent = error.message;
    }
  });

function updatePreviewImage() {
  // Get selected file, social platform name and rotation
  const file = document.querySelector("#input-file").files[0];
  const socialPlatformName = document.querySelector("#social-platform").value;
  const rotation = document.querySelector("#rotation").value;

  // Update preview image
  updatePreview(file, rotation, socialPlatformName);
}

const inputFile = document.querySelector("#input-file");
inputFile.addEventListener("change", updatePreviewImage);

const rotationSelect = document.querySelector("#rotation");
rotationSelect.addEventListener("change", updatePreviewImage);

const socialPlatformSelect = document.querySelector("#social-platform");
socialPlatformSelect.addEventListener("change", updatePreviewImage);

function updateOutput(input, output) {
  input.addEventListener("input", () => {
    output.textContent = input.value;
  });
}

document.addEventListener("DOMContentLoaded", () => {
  const brightnessInput = document.querySelector("#brightness");
  const brightnessOutput = document.querySelector("output[for=brightness]");
  updateOutput(brightnessInput, brightnessOutput);

  const contrastInput = document.querySelector("#contrast");
  const contrastOutput = document.querySelector("output[for=contrast]");
  updateOutput(contrastInput, contrastOutput);
});


const brightnessButton = document.querySelector("#brightness-button");
const brightnessContainer = document.querySelector("#brightness-container");
const contrastButton = document.querySelector("#contrast-button");
const contrastContainer = document.querySelector("#contrast-container");
const greyscaleButton = document.querySelector("#greyscale-button");
const greyscaleContainer = document.querySelector("#greyscale-container");

// Add event listeners to brightness and contrast buttons
function toggleContainer(container) {
  if (container.style.display === "none") {
    container.style.display = "block";
  } else {
    container.style.display = "none";
  }
}

brightnessButton.addEventListener("click", () => {
  toggleContainer(brightnessContainer);
});

contrastButton.addEventListener("click", () => {
  toggleContainer(contrastContainer);
});

greyscaleButton.addEventListener("click", () => {
  toggleContainer(greyscaleContainer);
});

// Get image file input and all inputs and buttons
const imageFileInput = document.querySelector("#input-file");
const inputs = document.querySelectorAll("input, button, select");

// Add event listener to image file input
function setInputsDisabled(disabled) {
  inputs.forEach(input => {
    input.disabled = disabled;
  });
}

// Add event listener to image file input
imageFileInput.addEventListener("change", () => {
  // Check if a file is selected
  if (imageFileInput.files.length > 0) {
    // Enable inputs and buttons
    setInputsDisabled(false);
  } else {
    // Disable inputs and buttons
    setInputsDisabled(true);
  }
});

// Disable inputs and buttons by default
setInputsDisabled(true);

// Enable the image file input by default
imageFileInput.disabled = false;

async function sendRotateResizeRequest(data) {
  return fetch("/get-image", {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify(data),
  });
}






