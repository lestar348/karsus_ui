cmake_minimum_required(VERSION 3.16)
project(lcdhat VERSION 1.0.0 LANGUAGES C)

set(CMAKE_C_STANDARD 99)
set(CMAKE_C_STANDARD_REQUIRED ON)

set(LCDHAT_BACKEND "DEV" CACHE STRING "Backend: DEV, WIRINGPI, BCM2835")
set_property(CACHE LCDHAT_BACKEND PROPERTY STRINGS DEV WIRINGPI BCM2835)

set(LCDHAT_PUBLIC_HEADERS
    include/lcdhat/lcdhat.h
)

set(LCDHAT_SOURCES
    src/lcdhat.c
    src/lcdhat_fonts.c
    src/lcdhat_log.c
    src/waveshare_compat/lib/Config/DEV_Config.c
    src/waveshare_compat/lib/LCD/LCD_1in44.c
    src/waveshare_compat/lib/GUI/GUI_Paint.c
    src/waveshare_compat/lib/GUI/GUI_BMP.c
    src/waveshare_compat/lib/GUI/KEY_APP.c
    src/waveshare_compat/lib/Fonts/font8.c
    src/waveshare_compat/lib/Fonts/font12.c
    src/waveshare_compat/lib/Fonts/font12CN.c
    src/waveshare_compat/lib/Fonts/font16.c
    src/waveshare_compat/lib/Fonts/font20.c
    src/waveshare_compat/lib/Fonts/font24.c
    src/waveshare_compat/lib/Fonts/font24CN.c
    src/waveshare_compat/lib/Fonts/font48.c
    src/waveshare_compat/lib/Fonts/font50.c
)

add_library(lcdhat SHARED ${LCDHAT_SOURCES} ${LCDHAT_PUBLIC_HEADERS})

set_target_properties(lcdhat PROPERTIES
    VERSION ${PROJECT_VERSION}
    SOVERSION 1
    PUBLIC_HEADER "${LCDHAT_PUBLIC_HEADERS}"
)

target_include_directories(lcdhat
    PUBLIC
        ${CMAKE_CURRENT_SOURCE_DIR}/include
    PRIVATE
        ${CMAKE_CURRENT_SOURCE_DIR}/src
        ${CMAKE_CURRENT_SOURCE_DIR}/src/waveshare_compat/lib/Config
        ${CMAKE_CURRENT_SOURCE_DIR}/src/waveshare_compat/lib/LCD
        ${CMAKE_CURRENT_SOURCE_DIR}/src/waveshare_compat/lib/GUI
        ${CMAKE_CURRENT_SOURCE_DIR}/src/waveshare_compat/lib/Fonts
)

if(LCDHAT_BACKEND STREQUAL "DEV")
    target_compile_definitions(lcdhat PRIVATE USE_DEV_LIB)
    find_path(LGPIO_INCLUDE_DIR lgpio.h)
    find_library(LGPIO_LIBRARY lgpio)
    if(LGPIO_INCLUDE_DIR AND LGPIO_LIBRARY)
        target_include_directories(lcdhat PRIVATE ${LGPIO_INCLUDE_DIR})
        target_link_libraries(lcdhat PRIVATE m pthread ${LGPIO_LIBRARY})
    else()
        message(WARNING "lgpio not found. Building with LCDHAT_USE_LGPIO_STUB (non-hardware environment).")
        target_compile_definitions(lcdhat PRIVATE LCDHAT_USE_LGPIO_STUB)
        target_include_directories(lcdhat PRIVATE ${CMAKE_CURRENT_SOURCE_DIR}/src/waveshare_compat/stub)
        target_link_libraries(lcdhat PRIVATE m pthread)
    endif()
elseif(LCDHAT_BACKEND STREQUAL "WIRINGPI")
    target_compile_definitions(lcdhat PRIVATE USE_WIRINGPI_LIB)
    target_link_libraries(lcdhat PRIVATE m wiringPi)
elseif(LCDHAT_BACKEND STREQUAL "BCM2835")
    target_compile_definitions(lcdhat PRIVATE USE_BCM2835_LIB)
    target_link_libraries(lcdhat PRIVATE m bcm2835)
else()
    message(FATAL_ERROR "Unsupported LCDHAT_BACKEND='${LCDHAT_BACKEND}'. Use DEV, WIRINGPI, BCM2835.")
endif()

add_subdirectory(examples/basic_demo)

install(TARGETS lcdhat
    LIBRARY DESTINATION lib
    ARCHIVE DESTINATION lib
    RUNTIME DESTINATION bin
    PUBLIC_HEADER DESTINATION include/lcdhat
)
