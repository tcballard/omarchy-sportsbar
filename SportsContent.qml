import QtQuick
import QtQuick.Layouts
import qs.Commons
import qs.Ui as Ui
import "Model.js" as Model

FocusScope {
    id:root
    property var service:null
    property var settings:({})
    property int page:0
    property string sport:"all"
    property string error:""
    signal changeSettings(var values)
    readonly property var favourites:Model.favourites(settings.teams)
    readonly property var rows:service ? service.followed.filter(function(m){return root.sport==="all" || root.sport===m.sport}) : []
    function toggleTeam(name,which) {
        var key=which+":"+String(name).toLowerCase()
        var next=favourites.slice()
        var index=next.indexOf(key)
        if(index>=0) next.splice(index,1)
        else if(next.length<40) next.push(key)
        else {error="Follow up to 40 teams.";return}
        changeSettings({teams:next.join("; ")})
    }
    ColumnLayout {
        anchors.fill:parent
        spacing:Style.space(14)
        RowLayout {
            Layout.fillWidth:true
            ColumnLayout {
                Layout.fillWidth:true
                Text {text:"SportsBar";font.pixelSize:Style.font.display;font.family:Style.font.family;font.bold:true;color:Color.foreground}
                Text {text:root.service && root.service.demo ? "FICTIONAL DEMO · NO ALERTS" : "Your teams. Back to work.";font.pixelSize:Style.font.bodySmall;font.family:Style.font.family;color:Color.accent}
            }
            Ui.Button {text:root.service && root.service.muted ? "Unmute" : "Mute";focusable:true;onClicked:if(root.service) root.service.muted=!root.service.muted}
        }
        RowLayout {
            Repeater {
                model:["Scores","Teams","Alerts & feeds"]
                Ui.Button {required property string modelData;required property int index;text:modelData;selected:root.page===index;focusable:true;onClicked:root.page=index}
            }
        }
        Flow {
            Layout.fillWidth:true
            spacing:Style.space(4)
            Repeater {
                model:["all","cricket","nfl","rugby","football"]
                Ui.Button {required property string modelData;text:modelData.toUpperCase();selected:root.sport===modelData;focusable:true;onClicked:root.sport=modelData}
            }
        }
        Text {visible:root.error!=="";Layout.fillWidth:true;text:root.error;color:Color.urgent;wrapMode:Text.Wrap;font.pixelSize:Style.font.body;textFormat:Text.PlainText}
        Flickable {
            Layout.fillWidth:true
            Layout.fillHeight:true
            contentHeight:body.implicitHeight
            clip:true
            boundsBehavior:Flickable.StopAtBounds
            Column {
                id:body
                width:parent.width
                spacing:Style.space(16)
                Column {
                    width:parent.width
                    spacing:Style.space(14)
                    visible:root.page===0
                    Text {
                        width:parent.width
                        visible:root.rows.length===0
                        text:!root.service ? "Waiting for the SportsBar service." : root.favourites.length===0 && !root.service.demo ? "Pick your teams to get started.\nOpen Teams to follow a club or country." : "No matches in the current feed for your teams.\nCheck Teams and Alerts & feeds for coverage."
                        font.family:Style.font.family;font.pixelSize:Style.font.body;color:Color.foreground;wrapMode:Text.Wrap
                    }
                    Ui.Button {
                        visible:!!root.settings.focusedEventId
                        text:"Clear bar focus"
                        focusable:true
                        onClicked:root.changeSettings({focusedEventId:"",focusedEventLabel:""})
                    }
                    Repeater {
                        model:root.rows
                        Column {
                            required property var modelData
                            width:parent.width
                            spacing:Style.space(6)
                            Text {
                                width:parent.width;textFormat:Text.PlainText
                                text:modelData.sport.toUpperCase()+" / "+modelData.state.toUpperCase()+(root.service && root.service.stale(modelData.sport) ? " · STALE" : "")
                                color:root.service && root.service.stale(modelData.sport) ? Color.urgent : Color.accent
                                font.pixelSize:Style.font.caption;font.family:Style.font.family
                            }
                            Text {width:parent.width;text:modelData.name;textFormat:Text.PlainText;wrapMode:Text.Wrap;font.pixelSize:Style.font.heading;font.family:Style.font.family;font.bold:true;color:Color.foreground}
                            Text {width:parent.width;text:Model.score(modelData);textFormat:Text.PlainText;wrapMode:Text.Wrap;font.pixelSize:Style.font.title;font.family:Style.font.family;color:Color.foreground}
                            Text {width:parent.width;text:modelData.detail || modelData.start;textFormat:Text.PlainText;wrapMode:Text.Wrap;font.pixelSize:Style.font.bodySmall;font.family:Style.font.family;color:Color.foreground;opacity:0.65}
                            Ui.Button {
                                text:root.settings.focusedEventId===modelData.id ? "✓ Focused in bar" : "Focus in bar"
                                selected:root.settings.focusedEventId===modelData.id
                                focusable:true
                                onClicked:root.changeSettings({focusedEventId:modelData.id,focusedEventLabel:Model.fixtureLabel(modelData)})
                            }
                            Rectangle {width:parent.width;height:1;color:Color.foreground;opacity:0.18}
                        }
                    }
                }
                Column {
                    width:parent.width;spacing:Style.space(10);visible:root.page===1
                    Text {width:parent.width;text:"Follow teams from the current fixtures, or add an exact team name below. Favourite teams are saved between sessions.";wrapMode:Text.Wrap;font.family:Style.font.family;font.pixelSize:Style.font.body;color:Color.foreground}
                    Ui.TextField {id:search;width:parent.width;placeholderText:"Search current teams"}
                    Repeater {
                        model:{
                            var names=[]
                            ;(root.service ? root.service.matches : []).forEach(function(m) {
                                if(root.sport!=="all" && root.sport!==m.sport) return
                                m.teams.forEach(function(t){var key=m.sport+":"+t.name;if(names.indexOf(key)<0 && t.name.toLowerCase().indexOf(search.text.toLowerCase())>=0) names.push(key)})
                            })
                            return names.sort().slice(0,80)
                        }
                        Ui.Button {
                            required property string modelData
                            text:(root.favourites.indexOf(modelData.toLowerCase())>=0 ? "✓ " : "+ ")+modelData
                            focusable:true
                            onClicked:root.toggleTeam(modelData.slice(modelData.indexOf(":")+1),modelData.split(":")[0])
                        }
                    }
                    Text {text:"ADD A TEAM";font.family:Style.font.family;font.pixelSize:Style.font.caption;color:Color.accent}
                    Ui.TextField {id:custom;width:parent.width;placeholderText:"Exact name, e.g. England or Gloucester";maximumLength:100;onAccepted:addTeam.clicked()}
                    Ui.Button {
                        id:addTeam
                        text:"Follow in "+(root.sport==="all" ? "selected sport" : root.sport)
                        enabled:root.sport!=="all" && custom.text.trim()!=="" && custom.text.indexOf(";")<0 && custom.text.indexOf(":")<0
                        focusable:true
                        onClicked:if(enabled) {if(root.favourites.indexOf(root.sport+":"+custom.text.trim().toLowerCase())<0) root.toggleTeam(custom.text.trim(),root.sport);custom.text=""}
                    }
                    Text {text:"FOLLOWING · CLICK TO REMOVE";font.family:Style.font.family;font.pixelSize:Style.font.caption;color:Color.accent}
                    Repeater {
                        model:root.favourites
                        Ui.Button {required property string modelData;text:"✓ "+modelData;focusable:true;onClicked:root.toggleTeam(modelData.slice(modelData.indexOf(":")+1),modelData.split(":")[0])}
                    }
                }
                Column {
                    width:parent.width;spacing:Style.space(10);visible:root.page===2
                    Repeater {
                        model:[{key:"notifications",label:"Desktop notifications"},{key:"wicketAlerts",label:"Wickets in followed cricket matches"},{key:"scoreAlerts",label:"Score changes in followed matches"},{key:"startAlerts",label:"Match started"},{key:"resultAlerts",label:"Match finished"},{key:"nfl",label:"NFL feed"},{key:"football",label:"Football feed"},{key:"rugby",label:"Rugby feed"},{key:"cricket",label:"Cricket feed"}]
                        Ui.Button {required property var modelData;text:(root.settings[modelData.key]!==false ? "✓ " : "○ ")+modelData.label;focusable:true;onClicked:{var change={};change[modelData.key]=root.settings[modelData.key]===false;root.changeSettings(change)}}
                    }
                    Ui.Button {text:(root.settings.fastPaidFeeds===true ? "✓ " : "○ ")+"Fast rugby polling (60 seconds)";focusable:true;onClicked:root.changeSettings({fastPaidFeeds:root.settings.fastPaidFeeds!==true})}
                    Text {width:parent.width;text:"Cricket uses ESPNcricinfo: no account or API key, refreshed every 30 seconds. Rugby still uses an optional keyed provider, every 20 minutes or 60 seconds with sufficient quota. Feed delays also apply.";wrapMode:Text.Wrap;font.family:Style.font.family;font.pixelSize:Style.font.bodySmall;color:Color.foreground}
                    Text {text:"FOOTBALL COMPETITION";font.family:Style.font.family;font.pixelSize:Style.font.caption;color:Color.accent}
                    Flow {
                        width:parent.width;spacing:Style.space(4)
                        Repeater {
                            model:[{id:"eng.1",name:"Premier League"},{id:"eng.2",name:"Championship"},{id:"eng.3",name:"League One"},{id:"eng.4",name:"League Two"},{id:"sco.1",name:"Scotland"},{id:"uefa.champions",name:"Champions League"},{id:"fifa.world",name:"World Cup"}]
                            Ui.Button {required property var modelData;text:modelData.name;selected:(root.settings.footballLeague || "eng.1")===modelData.id;focusable:true;onClicked:root.changeSettings({footballLeague:modelData.id})}
                        }
                    }
                    Text {width:parent.width;text:root.service ? root.service.statusLines() : "Service unavailable";textFormat:Text.PlainText;wrapMode:Text.Wrap;font.family:Style.font.family;font.pixelSize:Style.font.bodySmall;color:Color.foreground}
                    Text {width:parent.width;visible:!!root.service && root.service.notificationError!=="";text:root.service ? root.service.notificationError : "";wrapMode:Text.Wrap;color:Color.urgent;font.pixelSize:Style.font.bodySmall}
                    Ui.Button {text:(root.settings.demo===true ? "✓ " : "○ ")+"Fictional demo (no network or alerts)";focusable:true;onClicked:root.changeSettings({demo:root.settings.demo!==true})}
                }
            }
        }
        Text {Layout.fillWidth:true;text:"NFL / FOOTBALL / RUGBY / CRICKET";font.family:Style.font.family;font.pixelSize:Style.font.caption;color:Color.foreground;opacity:0.4}
    }
}
