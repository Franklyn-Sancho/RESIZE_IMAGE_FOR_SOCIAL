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