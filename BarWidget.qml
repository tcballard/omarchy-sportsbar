import QtQuick
import qs.Commons
import qs.Ui
import "Model.js" as Model

BarWidget {
    id: root
    moduleName: "io.github.tcballard.sportsbar"
    property var service: null
    readonly property string focusedId: String(setting("focusedEventId", ""))
    readonly property var displayedMatch: service ? Model.focusedMatch(service.followed, focusedId) : null
    readonly property string scoreLabel: {
        if (!service) return "Sports · unavailable"
        if (!displayedMatch) return focusedId ? String(setting("focusedEventLabel", "Match")) + " · unavailable" : "Sports"
        return Model.barScore(displayedMatch, vertical)
            + (vertical ? "" : service.demo ? " · DEMO" : service.stale(displayedMatch.sport) ? " · stale" : displayedMatch.state === "finished" ? " · final" : displayedMatch.state === "scheduled" ? " · upcoming" : "")
    }
    readonly property bool opened: panelLoader.item ? panelLoader.item.opened : false
    readonly property bool popoutSwitchClosing: panelLoader.item ? panelLoader.item.popoutSwitchClosing : false
    function open() {if(panelLoader.item) panelLoader.item.open()}
    function close() {if(panelLoader.item) panelLoader.item.close()}
    function closeForPopoutSwitch() {if(panelLoader.item) panelLoader.item.closeForPopoutSwitch()}
    function resolve() {
        var next = bar && bar.shell && typeof bar.shell.serviceFor === "function" ? bar.shell.serviceFor(moduleName) : null
        if (next !== service) service=next
        if (service) Qt.callLater(root.applyConfiguration)
        inject()
    }
    function applyConfiguration() { if (service) service.configure(settings) }
    function inject() {
        if(!panelLoader.item) return
        panelLoader.item.bar=bar
        panelLoader.item.settings=settings
        panelLoader.item.anchorItem=button
        panelLoader.item.hostWidget=root
        panelLoader.item.service=service
    }
    function persist(values) {
        var next=Object.assign({},settings,values,{id:moduleName})
        if (!bar || !bar.shell || !bar.shell.updateEntryInline(moduleName,next)) return false
        settings=next
        return true
    }
    onBarChanged: resolve()
    onSettingsChanged: resolve()
    onServiceChanged: inject()
    implicitWidth:button.implicitWidth
    implicitHeight:button.implicitHeight
    Timer {interval:1000;repeat:true;running:true;triggeredOnStart:true;onTriggered:root.resolve()}
    Loader {id:panelLoader;source:Qt.resolvedUrl("SportsPanel.qml");visible:false;onLoaded:root.inject()}
    WidgetButton {
        id:button
        anchors.fill:parent
        bar:root.bar
        text: root.scoreLabel
        tooltipText: (root.displayedMatch ? root.displayedMatch.name+"\n"+Model.score(root.displayedMatch)+"\n"+root.displayedMatch.detail : root.focusedId ? "Focused match is no longer in the current feed. Clear focus or choose another match." : "Choose a match to focus in the bar.")+"\nLeft: scores and teams · Right: mute alerts"
        onPressed:function(b) {
            if(b===Qt.RightButton && root.service) root.service.muted=!root.service.muted
            else if(b===Qt.LeftButton) root.opened ? root.close() : root.open()
        }
    }
}
