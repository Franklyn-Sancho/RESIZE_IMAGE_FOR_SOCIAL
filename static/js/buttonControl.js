const imageFileInput = document.querySelector("#input-file");
const inputs = document.querySelectorAll("input, button, select");

function setInputsDisabled(disabled) {
    inputs.forEach(input => {
        input.disabled = disabled;
    });
}

// Add event listener to image file input
imageFileInput.addEventListener("change", () => {

    if (imageFileInput.files.length > 0) {

        setInputsDisabled(false);
    } else {

        setInputsDisabled(true);
    }
});


setInputsDisabled(true);


imageFileInput.disabled = false;