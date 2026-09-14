"""Portable Qt component checks with explicit host doubles; not a live Omarchy test."""
import os,sys,tempfile,json
from pathlib import Path
os.environ.setdefault('QT_QPA_PLATFORM','offscreen')
os.environ.setdefault('QT_QUICK_BACKEND','software')
from PySide6.QtCore import QUrl,QTimer
from PySide6.QtGui import QGuiApplication
from PySide6.QtQuick import QQuickWindow
from PySide6.QtQml import QQmlApplicationEngine
repo=Path(__file__).resolve().parents[1]
app=QGuiApplication([])
tmp=tempfile.TemporaryDirectory();base=Path(tmp.name)
def write(p,s):
 p=base/p;p.parent.mkdir(parents=True,exist_ok=True);p.write_text(s)
write(Path('qs/Commons/qmldir'),'module qs.Commons\nsingleton Color 1.0 Color.qml\nsingleton Style 1.0 Style.qml\n')
write(Path('qs/Commons/Color.qml'),'pragma Singleton\nimport QtQuick\nQtObject {property color foreground:"#d8decf";property color background:"#171c1a";property color accent:"#b3cb92";property color urgent:"#f39a80"}')
write(Path('qs/Commons/Style.qml'),'pragma Singleton\nimport QtQuick\nQtObject {property var font:({family:"monospace",body:13,bodySmall:11,caption:10,title:16,heading:19,display:28});property var bar:({sizeHorizontal:28});function space(x){return x}}')
write(Path('qs/Ui/qmldir'),'module qs.Ui\nButton 1.0 Button.qml\nTextField 1.0 TextField.qml\nBarWidget 1.0 BarWidget.qml\nWidgetButton 1.0 WidgetButton.qml\nPanel 1.0 Panel.qml\nKeyboardPanel 1.0 KeyboardPanel.qml\n')
write(Path('qs/Ui/Button.qml'),'''import QtQuick
import QtQuick.Controls as C
import qs.Commons
C.Button {id:r;property bool selected:false;property bool focusable:false;activeFocusOnTab:focusable;implicitHeight:32;implicitWidth:label.implicitWidth+20;contentItem:Text{id:label;text:r.text;color:Color.foreground;font.family:Style.font.family;font.pixelSize:12;horizontalAlignment:Text.AlignHCenter;verticalAlignment:Text.AlignVCenter} background:Rectangle{color:r.selected ? "#35412f" : "#222a24";border.color:r.activeFocus ? Color.accent : "#3a4338";border.width:1}}
''')
write(Path('qs/Ui/TextField.qml'),'''import QtQuick
import QtQuick.Controls as C
import qs.Commons
C.TextField {color:Color.foreground;placeholderTextColor:"#879180";font.family:Style.font.family;background:Rectangle{color:"#222a24";border.color:"#3a4338"}}
''')
write(Path('qs/Ui/BarWidget.qml'),'import QtQuick\nItem {property var bar:null;property var settings:({});property string moduleName:"";property bool vertical:false;function setting(k,d){return settings[k]===undefined ? d : settings[k]}}')
write(Path('qs/Ui/WidgetButton.qml'),'import QtQuick\nItem {property var bar:null;property string text:"";property string tooltipText:"";implicitWidth:150;implicitHeight:28;signal pressed(int b)}')
write(Path('qs/Ui/Panel.qml'),'import QtQuick\nItem {property var bar:null;property var settings:({});property string moduleName:"";property bool manageIpc:false;property bool opened:false;property bool popoutSwitchClosing:false;function open(){opened=true} function close(){opened=false} function closeForPopoutSwitch(){opened=false}}')
write(Path('qs/Ui/KeyboardPanel.qml'),'import QtQuick\nItem {property var anchorItem:null;property var owner:null;property var bar:null;property bool open:false;property var focusTarget:null;property int contentWidth:540;property int contentHeight:660;function fittedContentWidth(w){return w} function fittedContentHeight(h){return h}}')
write(Path('Quickshell/Io/qmldir'),'module Quickshell.Io\nProcess 1.0 Process.qml\nSplitParser 1.0 SplitParser.qml\nIpcHandler 1.0 IpcHandler.qml\n')
write(Path('Quickshell/Io/Process.qml'),'import QtQuick\nItem {property bool running:false;property var command:[];property var stdout:null;signal exited(int code,int status)}')
write(Path('Quickshell/Io/SplitParser.qml'),'import QtQuick\nQtObject {signal read(string data)}')
write(Path('Quickshell/Io/IpcHandler.qml'),'import QtQuick\nQtObject {property string target:""}')
fixtures=json.loads((repo/'demo/fixtures/matches.json').read_text())
source=f'''import QtQuick
import QtQuick.Window
import "{repo.as_uri()}" as App
Window {{id:w;visible:true;width:480;height:620;color:"#171c1a"
 App.Service {{id:svc}}
 QtObject {{id:fakeBar;property var shell:QtObject {{function serviceFor(id) {{return svc}} function updateEntryInline(id,values) {{return true}}}}}}
 App.BarWidget {{id:widget;visible:false;bar:fakeBar;settings:({{demo:true,focusedEventId:"demo:cricket",focusedEventLabel:"ENG v AUS"}})}}
 Rectangle {{width:parent.width;height:32;color:"#222a24"
 Text {{anchors.centerIn:parent;text:widget.scoreLabel;color:"#d8decf";font.family:"monospace";font.pixelSize:13}}
 }}
 App.SportsContent {{id:content;anchors.fill:parent;anchors.margins:20;anchors.topMargin:64;anchors.bottomMargin:36;service:svc;sport:"all";settings:({{demo:true,focusedEventId:"demo:cricket"}});onChangeSettings:function(values) {{settings=Object.assign({{}},settings,values)}}}}
 Text {{anchors.bottom:parent.bottom;anchors.horizontalCenter:parent.horizontalCenter;anchors.bottomMargin:8;text:"Portable Qt preview · host controls simulated";color:"#879180";font.pixelSize:10}}
 function assertOk(v,msg) {{if(!v) throw new Error(msg)}}
 Component.onCompleted: {{
  content.toggleTeam("England","cricket")
  assertOk(content.settings.teams==="cricket:england","team choice saved through signal")
  content.toggleTeam("England","cricket")
  assertOk(content.settings.teams==="","team unfollow")
  svc.configure({{demo:true}})
  svc.buffer=JSON.stringify({json.dumps(fixtures)})
  svc.requestSport="demo";svc.requestGeneration=svc.generation;svc.complete(0)
  assertOk(svc.matches.length===4,"demo matches")
  assertOk(svc.notifications.length===0,"demo notifications")
  var old=svc.generation
  svc.configure({{demo:false,teams:"cricket:england"}})
  svc.requestGeneration=old;svc.buffer=JSON.stringify({json.dumps(fixtures)});svc.complete(0)
  assertOk(svc.matches.length===0,"late completion leaked")
  ;["nfl","football","rugby","cricket"].forEach(function(sport) {{assertOk(svc.interval(sport)===30000,"all sports default to thirty seconds")}})
  svc.deadlines.cricket=Date.now()+17000
  var cricketDeadline=svc.deadlines.cricket
  svc.requestGeneration=svc.generation;svc.requestSport="cricket";svc.buffer=JSON.stringify({{state:"ready",matches:[{json.dumps(fixtures['matches'][0])}]}});svc.complete(0)
  assertOk(svc.followed.length===1,"team selector")
  var focusGeneration=svc.generation
  assertOk(svc.deadlines.cricket===cricketDeadline,"completion preserves request-start deadline")
  svc.configure({{demo:false,teams:"cricket:england",focusedEventId:"demo:cricket",focusedEventLabel:"ENG v AUS"}})
  assertOk(svc.generation===focusGeneration && svc.followed.length===1,"focus reset live alert baseline")
  svc.buffer=JSON.stringify({{state:"offline",matches:[],message:"offline"}});svc.complete(0)
  assertOk(svc.matches.length===1,"last good scores retained")
  assertOk(svc.stale("cricket"),"offline scores marked stale")
  assertOk(Object.keys(svc.history.matches).length===0,"outage baseline cleared")
  svc.configure({{demo:true}});svc.requestGeneration=svc.generation;svc.requestSport="demo";svc.buffer=JSON.stringify({json.dumps(fixtures)});svc.complete(0)
  console.log("QML lifecycle checks passed")
 }}
}}'''
write(Path('Harness.qml'),source)
engine=QQmlApplicationEngine();engine.addImportPath(str(base));warnings=[]
engine.warnings.connect(lambda items:warnings.extend(str(i.toString()) for i in items))
engine.load(QUrl.fromLocalFile(str(base/'Harness.qml')))
if not engine.rootObjects():sys.exit(1)
def finish():
 if '--screenshot' in sys.argv:
  destination=repo/'docs/portable-preview.png'
  engine.rootObjects()[0].grabWindow().save(str(destination))
  print(destination)
 if warnings:
  print('\n'.join(warnings),file=sys.stderr)
 app.exit(1 if warnings else 0)
QTimer.singleShot(800,finish)
sys.exit(app.exec())
