add_executable(lcdhat_basic_demo main.c)

target_link_libraries(lcdhat_basic_demo PRIVATE lcdhat)

target_include_directories(lcdhat_basic_demo PRIVATE
    ${CMAKE_CURRENT_SOURCE_DIR}/../../include
)
