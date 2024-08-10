class FileByteReader extends HTMLInputElement {
  constructor() {
    super();
  }
  connectedCallback() {
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

  onChange() {
    const fileReader = new FileReader();
    fileReader.addEventListener('loadend', e => this.emit('loaded', new Int8Array(fileReader.result)));
    fileReader.readAsArrayBuffer(this.files[0]);
  }
}
customElements.define("file-byte-reader", FileByteReader, { extends: 'input'});