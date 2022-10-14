mode=""

WOKWI_ID = ""
ifeq ($(mode),blinky)
	WOKWI_ID = "345500331909579346"
endif

run:
	cp ./src/examples/blinky.rs ./src/main.rs ; \
	export WOKWI_PROJECT_ID=$(WOKWI_ID) ; \
	./scripts/run-wokwi.sh