FROM japaric/aarch64-unknown-linux-gnu:v0.1.14

RUN apt-get update && \
    apt-get install unzip -y && \
    apt-get install curl -y && \
    apt-get install git -y 

RUN git clone --branch zjshen https://github.com/zjshen14/libra.git && \
    cd libra && \
    echo y | scripts/dev_setup.sh

RUN apt-get purge cmake -y && \
    curl -L https://github.com/Kitware/CMake/releases/download/v3.14.5/cmake-3.14.5-Linux-x86_64.tar.gz > cmake-3.14.5-Linux-x86_64.tar.gz && \
    tar -xzf cmake-3.14.5-Linux-x86_64.tar.gz && \
    cd cmake-3.14.5-Linux-x86_64 && \
    cp -r bin /usr/ && \
    cp -r share /usr/
