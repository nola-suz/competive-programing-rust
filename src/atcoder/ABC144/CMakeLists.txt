cmake_minimum_required(VERSION 3.4)
project(ABC CXX)

# ディレクトリ内のソースをすべてプロジェクトにぶちこむマクロ
MACRO(ADD_ALL_DIR curdir)
    FILE(GLOB children RELATIVE ${curdir} ${curdir}/*)
    FOREACH(child ${children})
        IF(${curdir}/${child} MATCHES ".cpp$")
            get_filename_component(FILE_NAME ${child} NAME_WE)
            ADD_EXECUTABLE(ABC_${FILE_NAME} ${curdir}/${child})
        ENDIF()
    ENDFOREACH()
ENDMACRO()

set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -std=c++11")

ADD_ALL_DIR(${ABC_SOURCE_DIR})