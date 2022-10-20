mode=""

WOKWI_ID = ""
example_name = ""
ifeq ($(mode),led)
	WOKWI_ID = "345500331909579346"
	example_name=$(mode)
else ifeq ($(mode),button)
	WOKWI_ID = "345500331909579346"
	example_name=$(mode)
else ifeq ($(mode),max7219)
	WOKWI_ID = "345500331909579346"
	example_name=$(mode)
else ifeq ($(mode),switch)
	WOKWI_ID = "345500331909579346"
	example_name=$(mode)
else ifeq ($(mode),ssd1306)
	WOKWI_ID = "345500331909579346"
	example_name=$(mode)
else ifeq ($(mode),rgb2)
	WOKWI_ID = "345500331909579346"
	example_name=$(mode)
else ifeq ($(mode),serial)
	WOKWI_ID = "345500331909579346"
	example_name=$(mode)
else ifeq ($(mode),rgb)
	WOKWI_ID = "345565422673723988"
	example_name=$(mode) --features="smartled"
else ifeq ($(mode),i2c_display)
	WOKWI_ID = "345568492541444691"
	example_name=$(mode)
else ifeq ($(mode),pulse_control)
	WOKWI_ID = ""
	example_name=$(mode)
endif

run:
	export WOKWI_PROJECT_ID=$(WOKWI_ID) ; \
	./scripts/run-wokwi.sh "" $(example_name)
	