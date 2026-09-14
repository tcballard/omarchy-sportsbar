import QtQuick
import qs.Commons
import qs.Ui

Panel {
    id:root
    moduleName:"io.github.tcballard.sportsbar"
    manageIpc:false
    property var anchorItem:null
    property var hostWidget:null
    property var service:null
    KeyboardPanel {
        id:popup
        anchorItem:root.anchorItem
        owner:root.hostWidget || root
        bar:root.bar
        open:root.opened
        focusTarget:content
        contentWidth:popup.fittedContentWidth(Style.space(440))
        contentHeight:popup.fittedContentHeight(Style.space(520))
        SportsContent {
            id:content
            anchors.fill:parent
            service:root.service
            settings:root.settings
            onChangeSettings:function(values) {
                if (!root.hostWidget || !root.hostWidget.persist(values)) content.error="Could not save settings through this bar."
            }
            Keys.onEscapePressed:root.close()
        }
    }
}
