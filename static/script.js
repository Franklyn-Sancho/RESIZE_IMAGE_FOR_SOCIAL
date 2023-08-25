// Utility functions

function readFormData() {
    let imageFile = document.getElementById("input-file").files[0];
    let socialPlatformName = document.getElementById("social-platform").value;
    let rotation = document.getElementById("rotation").value;
  
    // Update preview image
    updatePreview(imageFile, rotation);
  
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
  
  function updatePreview(imageFile, rotation) {
    // Create object URL from file
    const objectURL = URL.createObjectURL(imageFile);
  
    // Set preview image src to object URL
    const previewImage = document.querySelector("#preview-image");
    previewImage.src = objectURL;
  
    // Apply rotation to preview image
    applyRotation(previewImage, rotation);
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
    // Get selected file
    const file = inputFile.files[0];
  
    // Update preview image
    updatePreview(file);
  });
  
  const rotationSelect = document.querySelector("#rotation");
  rotationSelect.addEventListener("change", () => {
    // Get selected file and rotation
    const file = document.querySelector("#input-file").files[0];
    const rotation = rotationSelect.value;
  
    // Update preview image
    updatePreview(file, rotation);
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
  


