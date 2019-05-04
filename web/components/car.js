const baseURL = base(import.meta.url)
const html = (strings, ...parts) => new DOMParser().parseFromString(
	parts.reduce((tpl, value, i) => `${tpl}${strings[i]}${value}`, '')
		.concat(strings[parts.length]), 'text/html').querySelector('template')
let controller = null

const ctrlTpl = html`<template>
<style>
:host { 
  align-items: center;
  background: linear-gradient(to bottom, #2a2a2a, #333 25%, #333);
  display: flex;
  justify-content: space-around;
}
</style>
<slot></slot>
</template>`
export class Control extends HTMLElement {
	constructor() {
		super()
		this.attachShadow({mode: 'open'})
		this.shadowRoot.append(ctrlTpl.content.cloneNode(true))
		this.steer = 0
		this.accel = 0
		this._update = this.update.bind(this)
	}

	connectedCallback() {
		this.$wheel = this.querySelector('car-wheel')
		requestAnimationFrame(this._update)
	}

	update() {
		if (controller)
			this.steer = controller.axes[0].toFixed(2)
		if (this.steer != this.$wheel.value)
			this.$wheel.value = this.steer
		requestAnimationFrame(this._update)
	}
}
customElements.define('car-control', Control)

const wheelTpl = html`<template>
<style>
:host {
  background-position: center;
  background-size: contain;
  background-image: url(${baseURL}/shadow.svg);
  contain: strict;
  display: flex;
  height: 50vmin;
  overflow: hidden;
  width: 50vmin;
}
#wheel { will-change: transform; height: 92%; margin: auto; position: relative; top: -3px; }
</style>
<img id="wheel" src="${baseURL}/wheel.svg" />
</template>`

export class Wheel extends HTMLElement {
	static get maxTurn() { return 0.4 }

	constructor() {
		super()
		const sdw = this.attachShadow({mode: 'open'})
		sdw.append(wheelTpl.content.cloneNode(true))
		this.$img = sdw.querySelector('#wheel')
		this._val = 0
	}

	get value() { return this._val }
	set value(val) {
		this._val = val
		this.$img.style.transform = `rotate(${val * Wheel.maxTurn}turn)`
	}
}
customElements.define('car-wheel', Wheel)

addEventListener('gamepadconnected', ({gamepad}) => controller = gamepad)
addEventListener('gamepaddisconnected', () => controller = null)

function base(url) {
	let parts = url.split('/')
	parts.pop()
	return parts.join('/')
}
