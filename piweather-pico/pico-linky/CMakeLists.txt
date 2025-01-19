project(pico-linky)
add_executable(pico-linky main.c)
target_link_libraries(pico-linky PRIVATE pico-stdlib)