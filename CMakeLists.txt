cmake_minimum_required(VERSION 3.20)

include(FetchContent)
FetchContent_Declare(
    pico-sdk
    URL https://github.com/raspberrypi/pico-sdk/releases/download/2.1.0/pico-sdk-2.1.0.tar.gz
    DOWNLOAD_EXTRACT_TIMESTAMP ON
)
FetchContent_MakeAvailable(pico-sdk)

# == DO NOT EDIT THE FOLLOWING LINES for the Raspberry Pi Pico VS Code Extension to work ==
if(WIN32)
    set(USERHOME $ENV{USERPROFILE})
else()
    set(USERHOME $ENV{HOME})
endif()
set(sdkVersion 2.1.0)
set(toolchainVersion 13_3_Rel1)
set(picotoolVersion 2.1.0)
#set(picoVscode ${USERHOME}/.pico-sdk/cmake/pico-vscode.cmake)
if (EXISTS ${picoVscode})
    include(${picoVscode})
endif()
# ====================================================================================
set(PICO_BOARD pico2_w CACHE STRING "Board type")

# Pull in Raspberry Pi Pico SDK (must be before project)
include(${pico-sdk_SOURCE_DIR}/pico_sdk_init.cmake)

project(piweather-pico)

# Initialise the Raspberry Pi Pico SDK
pico_sdk_init()

add_subdirectory(piweather-pico/)
