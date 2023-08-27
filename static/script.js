function readFormData() {
  let imageFile = document.getElementById("input-file").files[0];
  let socialPlatformName = document.getElementById("social-platform").value;
  let rotation = document.getElementById("rotation").value;

  // Update preview image
  updatePreview(imageFile, rotation, socialPlatformName);

  return { imageFile, socialPlatformName, rotation };
}

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

// Preview image functions

function updatePreview(imageFile, rotation, socialPlatformName) {
  // Create object URL from file
  const objectURL = URL.createObjectURL(imageFile);

  // Set preview image src to object URL
  const previewImage = document.querySelector("#preview-image");
  previewImage.src = objectURL;

  // Apply rotation to preview image
  applyRotation(previewImage, rotation);

  // Apply resizing to preview image
  applyResizing(previewImage, socialPlatformName);
}

function applyRotation(imageElement, rotation) {
  // Define rotation map
  const rotationMap = {
    Right90: "rotate(90deg)",
    Left90: "rotate(-90deg)",
    HalfCircle: "rotate(180deg)",
    None: "none",
  };

  // Get transform value from rotation map
  const transformValue = rotationMap[rotation] || "none";

  // Apply transform to image element
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

// Event listeners

document
  .getElementById("rotate-resize-form")
  .addEventListener("submit", async (event) => {
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

    document.getElementById("result").textContent =
      "Imagem baixada com sucesso!";
  });

const inputFile = document.querySelector("#input-file");
inputFile.addEventListener("change", () => {
  // Get selected file and social platform name
  const file = inputFile.files[0];
  const socialPlatformName =
    document.querySelector("#social-platform").value;

  // Update preview image
  updatePreview(file, undefined, socialPlatformName);
});

const rotationSelect = document.querySelector("#rotation");
rotationSelect.addEventListener("change", () => {
  // Get selected file and rotation
  const file = document.querySelector("#input-file").files[0];
  const rotation = rotationSelect.value;

  // Update preview image
  updatePreview(file, rotation);
});

const socialPlatformSelect =
  document.querySelector("#social-platform");
socialPlatformSelect.addEventListener("change", () => {
  // Get selected file and social platform name
  const file = document.querySelector("#input-file").files[0];
  const socialPlatformName =
    socialPlatformSelect.value;

  // Update preview image
  updatePreview(file, undefined, socialPlatformName);
});

// Server communication functions

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




