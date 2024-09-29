function showElementById(id) {
  document.getElementById(id).style.display = "block"
}

function showChoice() {
  document.getElementById("choice").style.display = "block";
}

function nextLine(callback) {
  const lines = document.getElementById("dialog-text").children;
  let found = false;

  for (let i = 0; i < lines.length; i++) {
    const line = lines[i];

    if (line.style.display !== "none" && i != lines.length - 1) {
      found = true;
      line.style.display = "none";
    } else if (found) {
      line.style.display = "block";
      if (callback && i == lines.length - 1) callback()
      break;
    }
  }

}
