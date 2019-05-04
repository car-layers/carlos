out=web/carlos

build() {
	ARGS="--no-typescript -t web"
	ARGS=$ARGS' --dev'
	wasm-pack build $ARGS $1 && \
		mv $1/pkg/*.{js,wasm} $out
}

rm -rf $out && \
	mkdir -p $out && \
	build driver && \
	build fake-car
