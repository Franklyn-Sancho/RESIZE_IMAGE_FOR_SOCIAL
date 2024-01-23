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