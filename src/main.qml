import QtQuick
import QtQuick.Controls

ApplicationWindow {
    width: 400
    height: 200
    visible: true

    Text {
        anchors.centerIn: parent
        horizontalAlignment: Text.AlignHCenter
        font.pointSize: 30
        text: "Hello\nQML powered\nRust-World!"
    }
}
