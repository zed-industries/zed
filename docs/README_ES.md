<h4 align="center">
    <p>
        <a href="../README.md">English</b> |
        <b>Español</b> |
        <a href="./README_CH.md">普通话</a>
    </p>
</h4>

# Zed

[![CI](https://github.com/zed-industries/zed/actions/workflows/ci.yml/badge.svg)](https://github.com/zed-industries/zed/actions/workflows/ci.yml)

Bienvenido a Zed, un editor de código multijugador de alto rendimiento de los creadores de [Atom](https://github.com/atom/atom) y [Tree-sitter](https://github.com/tree-sitter/tree-sitter).

--------

### Instalación


<a href="https://repology.org/project/zed-editor/versions">
    <img src="https://repology.org/badge/vertical-allrepos/zed-editor.svg?minversion=0.143.5" alt="Packaging status" align="right">
</a>

En macOS y Linux puedes [descargar Zed directamente](https://zed.dev/download) o [instale Zed a través de su administrador de paquetes local](https://zed.dev/docs/linux#installing-via-a-package-manager).

Otras plataformas aún no están disponibles:

- Windows ([tracking issue](https://github.com/zed-industries/zed/issues/5394))
- Web ([tracking issue](https://github.com/zed-industries/zed/issues/5396))

### Desarrollando Zed

- [Construyendo Zed para macOS](./docs/src/development/macos.md)
- [Construyendo Zed para Linux](./docs/src/development/linux.md)
- [Construcción de Zed para Windows] (./docs/src/development/windows.md)
- [Ejecución de colaboración local] (./docs/src/development/local-collaboration.md)

### Contribuyendo

Consulte [CONTRIBUTING.md](../CONTRIBUTING.md) para conocer las formas en que puede contribuir a Zed.

Además... ¡estamos contratando! Consulte nuestra página de [trabajos](https://zed.dev/jobs) para conocer los puestos vacantes.

### Licencias

La información de licencia para dependencias de terceros se debe proporcionar correctamente para que CI pase.

Usamos [`cargo-about`](https://github.com/EmbarkStudios/cargo-about) para cumplir automáticamente con las licencias de código abierto. Si CI falla, verifique lo siguiente:

- ¿Aparece el error "no se ha especificado ninguna licencia" para una caja que has creado? Si es así, agregue `publish = false` bajo `[paquete]` en el Cargo.toml de su caja.
- ¿Aparece el error "no se pudieron satisfacer los requisitos de licencia" para una dependencia? Si es así, primero determine qué licencia tiene el proyecto y si este sistema es suficiente para cumplir con los requisitos de esta licencia. Si no está seguro, consulte a un abogado. Una vez que haya verificado que este sistema es aceptable, agregue el identificador SPDX de la licencia a la matriz "aceptado" en "script/licenses/zed-licenses.toml".
- ¿`cargo-about` no puede encontrar la licencia para una dependencia? Si es así, agregue un campo de aclaración al final de `script/licenses/zed-licenses.toml`, como se especifica en el [libro sobre carga](https://embarkstudios.github.io/cargo-about/cli/ generar/config.html#crate-configuration).