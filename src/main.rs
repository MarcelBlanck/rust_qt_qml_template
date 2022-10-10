use cpp::*;
use qmetaobject::*;
use std::env;

cpp! {{
    #include <QtQml/QQmlEngine>
}}

fn main() {
    let mut engine = QmlEngine::new();
    engine.load_file(format!("{}/src/main.qml", env!("CARGO_MANIFEST_DIR")).into());
    engine.exec();
}
