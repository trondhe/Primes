cmake_minimum_required(VERSION 3.10)

project(prime CXX)

add_executable(prime)
set(CMAKE_INTERPROCEDURAL_OPTIMIZATION true)

target_sources(prime
    PRIVATE
        main.cpp
)


add_library(primelib STATIC)

target_sources(primelib
    PRIVATE
        prime.cpp
)

target_link_libraries(prime
    PUBLIC
        primelib
)
