import QtQuick
import Quickshell.Io
import "Model.js" as Model

Item {
    id: root
    property var shell: null
    property var manifest: null
    property string omarchyPath: ""
    property var options: ({})
    property bool configured: false
    property bool busy: false
    property var feeds: ({})
    property var deadlines: ({})
    property var failures: ({})
    property var history: Model.tracker()
    property var notifications: []
    property var recent: []
    property string notificationError: ""
    property int generation: 0
    property int requestGeneration: 0
    property string requestSport: ""
    property string buffer: ""
    property bool overflow: false
    property bool muted: false
    property double now: Date.now()
    readonly property bool demo: options.demo === true
    readonly property var selected: Model.favourites(options.teams)
    readonly property var matches: {
        var result = []
        for (var sport in feeds) result = result.concat(feeds[sport].matches || [])
        return result
    }
    readonly property var followed: matches.filter(function(m) { return root.demo || Model.follows(m, root.selected) })
    readonly property var live: followed.filter(function(m) { return m.state === "live" })
    function configure(settings) {
        var next = JSON.parse(JSON.stringify(settings || {}))
        // Focusing another event must not reset polling or lose an alert baseline.
        delete next.focusedEventId
        delete next.focusedEventLabel
        if (configured && JSON.stringify(next) === JSON.stringify(options)) return
        options = next
        configured = true
        generation++
        feeds = ({})
        deadlines = ({})
        failures = ({})
        history = Model.tracker()
        notifications = []
        Qt.callLater(tick)
    }
    function interval(sport) {
        return 30000
    }
    function sports() {
        if (demo) return ["demo"]
        return ["nfl","football","rugby","cricket"].filter(function(s) { return options[s] !== false })
    }
    function tick() {
        now = Date.now()
        if (!configured || busy) return
        var list = sports()
        for (var i=0; i<list.length; i++) {
            var sport = list[i]
            if ((deadlines[sport] || 0) > now) continue
            requestSport = sport
            requestGeneration = generation
            buffer = ""
            overflow = false
            deadlines[sport] = now + interval(sport)
            poll.command = ["/usr/bin/timeout", "55", "sportsbar-feed", demo ? "--demo" : sport, String(sport === "rugby" ? (options.rugbyLeague || "267979") : (options.footballLeague || "eng.1"))]
            busy = true
            poll.running = true
            return
        }
    }
    function complete(code) {
        busy = false
        if (requestGeneration !== generation) { Qt.callLater(tick); return }
        var result
        try {
            if (code !== 0 || overflow) throw new Error("helper")
            result = JSON.parse(buffer)
            if (!Array.isArray(result.matches) || typeof result.state !== "string") throw new Error("schema")
        } catch(e) {
            result = {sport:requestSport,state:"failed",message:"Feed helper failed. Build/install sportsbar-feed and check provider setup.",matches:[]}
        }
        var next = Object.assign({},feeds)
        var success = result.state === "ready" || result.state === "empty"
        if (success) {
            result.updated = Date.now()
            failures[requestSport] = 0
            var events = Model.ingest(history,result.matches,selected,Date.now(),interval(requestSport)*2+15000,!demo && !muted && options.notifications !== false)
            events = events.filter(function(e) { return options[e.kind + "Alerts"] !== false })
            recent = events.concat(recent).slice(0,30)
            notifications = notifications.concat(events).slice(0,20)
            dispatch()
        } else {
            Object.keys(history.matches).forEach(function(id) { if(history.matches[id].match.sport===requestSport) delete history.matches[id] })
            failures[requestSport] = Math.min(6,(failures[requestSport] || 0)+1)
            deadlines[requestSport] = Date.now()+Math.min(3600000,interval(requestSport)*Math.pow(2,failures[requestSport]))
            result.matches = feeds[requestSport] ? feeds[requestSport].matches || [] : []
            result.updated = feeds[requestSport] ? feeds[requestSport].updated || 0 : 0
        }
        next[requestSport] = result
        feeds = next
        Qt.callLater(tick)
    }
    function stale(sport) {
        var feed = feeds[demo ? "demo" : sport]
        return !feed || ["ready","empty"].indexOf(feed.state)<0 || now-(feed.updated || 0)>interval(sport)+30000
    }
    function statusLines() {
        return sports().map(function(s) {
            var f=feeds[s]
            return s.toUpperCase()+": "+(f ? f.state + (f.message ? " — "+f.message : "") : "Waiting for first update")
        }).join("\n")
    }
    function dispatch() {
        if (notify.running || !notifications.length) return
        if (muted || options.notifications === false || demo) {notifications=[];return}
        var queue=notifications.slice()
        var event=queue.shift()
        notifications=queue
        notify.command=["/usr/bin/timeout","8","notify-send","--app-name=SportsBar","--expire-time=8000","--",Model.clean(event.title),Model.clean(event.body)]
        notify.running=true
    }
    onMutedChanged: if (muted) notifications=[]
    Process {
        id: poll
        stdout: SplitParser {
            onRead: function(data) {
                if (root.buffer.length+data.length>1048576) {root.overflow=true;poll.running=false}
                else root.buffer+=data
            }
        }
        onExited: function(code,status) {root.complete(code)}
    }
    Process {
        id: notify
        onExited: function(code,status) {
            root.notificationError=code===0 ? "" : "Desktop notification failed; check libnotify and the notification service."
            notificationDelay.restart()
        }
    }
    Timer {id:notificationDelay;interval:1200;onTriggered:root.dispatch()}
    Timer {interval:1000;running:root.configured;repeat:true;onTriggered:root.tick()}
    IpcHandler {
        target:"io.github.tcballard.sportsbar"
        function refresh(): void { root.tick() }
        function status(): string {return JSON.stringify({configured:root.configured,loading:poll.running,followed:root.followed.length,feeds:root.statusLines(),muted:root.muted})}
    }
    Component.onDestruction: {generation++;poll.running=false;notify.running=false;notifications=[]}
}
