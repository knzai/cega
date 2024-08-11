class FileByteReader extends HTMLInputElement {
  connectedCallback() {
    this.type = "file";
    this.addEventListener('change', this.onChange);
  }

  emit (type, detail = {}) {
    let event = new CustomEvent(`file-byte-reader:${type}`, {
      bubbles: false,
      cancelable: false,
      detail: detail
    });
    return this.dispatchEvent(event);
  }

  onFileLoad(event) {
    this.data = new Int8Array(event.target.result);
    this.emit('loaded', this.data);
  }

  onChange() {
    if (this.files.length == 0) { return }
    Array.from(this.files).forEach(file => {
      const fileReader = new FileReader();
      fileReader.addEventListener('loadend', e => this.onFileLoad(e));
      fileReader.readAsArrayBuffer(file);
    });
  }
}
customElements.define("file-byte-reader", FileByteReader, { extends: 'input'});