FROM registry.gitlab.com/islandoftex/images/texlive:latest

ARG TARGETARCH

ENV LANG=C.UTF-8 \
    LC_ALL=C.UTF-8 \
    TERM=dumb

ARG BUILD_DATE
ARG GITLATEXDIFF_VERSION=1.6.0

WORKDIR /home

RUN mkdir -p /usr/share/man/man1

RUN apt-get update -q && \
    apt-get install -qqy -o=Dpkg::Use-Pty=0 --no-install-recommends less wget && \
    apt-get install -qqy -o=Dpkg::Use-Pty=0 ruby poppler-utils && gem install bundler && \
    apt-get install -qqy -o=Dpkg::Use-Pty=0 --no-install-recommends libfile-copy-recursive-perl openssh-client  && \
    apt-get install -qqy -o=Dpkg::Use-Pty=0 --no-install-recommends graphviz inkscape && \
    apt-get install -qqy -o=Dpkg::Use-Pty=0 --no-install-recommends latexml && \
    apt-get install -qqy -o=Dpkg::Use-Pty=0 fig2dev && \
    apt-get install -qqy -o=Dpkg::Use-Pty=0 fonts-inconsolata && \
    apt-get install -qqy -o=Dpkg::Use-Pty=0 bibtool && \
    apt-get install -qqy -o=Dpkg::Use-Pty=0 python3-pip && \
    apt-get install -qqy -o=Dpkg::Use-Pty=0 gnuplot && \
    apt-get --purge remove -qqy .\*-doc$ && \
    rm -rf /var/lib/apt/lists/* && apt-get clean

RUN cd /tmp && \
    git clone --depth=1 https://gitlab.com/islandoftex/texmf/depp.git && \
    cd depp && \
    l3build install && \
    cd .. && \
    rm -rf /tmp/depp

RUN curl -o /home/pandoc.deb -sSL https://github.com/jgm/pandoc/releases/download/3.6.4/pandoc-3.6.4-1-$TARGETARCH.deb && dpkg -i pandoc.deb && rm pandoc.deb

RUN curl -o /home/plantuml.zip -sSL https://github.com/plantuml/plantuml/releases/download/v1.2025.2/plantuml-asl-1.2025.2.jar && \
  unzip -q plantuml.zip && \
  rm plantuml.zip

ENV PLANTUML_JAR=/home/plantuml.jar

RUN luaotfload-tool --update

COPY ./tmp/latex-compiler /opt/latex-compiler

ENTRYPOINT ["/opt/latex-compiler", "latex-compiler"]
