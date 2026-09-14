const {test} = require('node:test');
const assert = require('node:assert/strict');
const fs = require('node:fs');
const vm = require('node:vm');
const path = require('node:path');

// Run production service methods with a controlled clock and inert processes.
// Real Quickshell lifecycle and process delivery are separate desktop gates.
function harness() {
    const source = fs.readFileSync(path.join(__dirname, '../Service.qml'), 'utf8');
    const methods = source.slice(source.indexOf('    function configure('), source.indexOf('    onMutedChanged:'));
    let clock = 100000;
    const s = {Date:{now:()=>clock}, Qt:{callLater:()=>{}}, Model:{tracker:()=>({matches:{}}), clean:x=>x, ingest:()=>[]},
        overflow:false, options:{}, configured:true, busy:false, demo:false, generation:1, requestGeneration:1,
        feeds:{}, retryDeadlines:{}, deadlines:{}, failures:{}, history:{matches:{}}, selected:[], recent:[], notifications:[],
        poll:{running:false}, notify:{running:false}, notificationDelay:{running:false}, muted:false};
    vm.createContext(s); vm.runInContext(methods,s);
    return {s, advance:ms=>{clock+=ms;}, time:()=>clock};
}
test('slow successful requests serve every sport fairly without overlap',()=>{
    const {s,advance}=harness(), starts=[];
    for(let i=0;i<25;i++) {
        s.busy=false; s.tick(); starts.push(s.requestSport);
        const deadline=s.deadlines[s.requestSport];
        s.tick(); assert.equal(s.deadlines[s.requestSport],deadline);
        advance(12000);
        s.buffer=JSON.stringify({state:"ready",matches:[]}); s.complete(0);
    }
    for(let i=0;i<starts.length;i++) assert.equal(starts[i],['nfl','football','rugby','cricket'][i%4]);
});
test('idle refresh respects deadlines and disabled sports are skipped',()=>{
    const {s,advance}=harness(); s.options={football:false,rugby:false,cricket:false};
    s.tick(); assert.equal(s.requestSport,'nfl'); s.busy=false; s.poll.running=false;
    advance(29999); s.tick(); assert.equal(s.poll.running,false);
    advance(1); s.tick(); assert.equal(s.poll.running,true);
});
test('server wait is not capped by local backoff or shortened by manual refresh',()=>{
    const {s,time,advance}=harness(); s.requestSport='cricket';
    s.buffer=JSON.stringify({state:'rate-limited',matches:[],retryAfterSec:7200}); s.complete(0);
    assert.equal(s.deadlines.cricket,time()+7200000);
    s.options={nfl:false,football:false,rugby:false}; advance(7199999); s.tick();
    assert.equal(s.poll.running,false); advance(1); s.tick(); assert.equal(s.poll.running,true);
});
test('invalid or shorter server waits retain local exponential backoff',()=>{
    for(const wait of [null,-1,'7200',0,1]) {
        const {s,time}=harness(); s.requestSport='cricket';
        s.buffer=JSON.stringify({state:'failed',matches:[],retryAfterSec:wait}); s.complete(0);
        assert.equal(s.deadlines.cricket,time()+60000); s.complete(0);
        assert.equal(s.deadlines.cricket,time()+120000);
    }
});
test('late completion cannot install a retry deadline in a newer generation',()=>{
    const {s}=harness(); s.requestSport='cricket'; s.requestGeneration=0;
    s.buffer=JSON.stringify({state:'rate-limited',matches:[],retryAfterSec:7200}); s.complete(0);
    assert.equal(s.deadlines.cricket,undefined);
});
test('notification spacing cannot be bypassed by a second feed completion',()=>{
    const {s}=harness(); s.notifications=[{title:'Score',body:'1–0'}];
    s.notificationDelay.running=true; s.dispatch(); assert.equal(s.notify.running,false);
    assert.equal(s.notifications.length,1);
    s.notificationDelay.running=false; s.dispatch(); assert.equal(s.notify.running,true);
    assert.equal(s.notifications.length,0);
});

test('changing settings cannot reset a server cooldown',()=>{
    const {s}=harness(); s.requestSport='cricket';
    s.buffer=JSON.stringify({state:'rate-limited',matches:[],retryAfterSec:7200}); s.complete(0);
    s.configure({nfl:false,football:false,rugby:false,notifications:false}); s.tick();
    assert.equal(s.poll.running,false);
});

test('live capture refuses non-demo state and never overwrites output',()=>{
    const os=require('node:os'), {spawnSync}=require('node:child_process');
    const tmp=fs.mkdtempSync(path.join(os.tmpdir(),'sportsbar-capture-'));
    const runner=path.resolve(__dirname,'../demo/capture-live');
    const shim=(name,body)=>fs.writeFileSync(path.join(tmp,name),'#!/bin/bash\n'+body,{mode:0o755});
    try {
        shim('omarchy-shell','echo "${DEMO_STATUS}"\n');
        shim('grim','printf "fixture image" > "${@: -1}"\n');
        const out=path.join(tmp,'capture.png');
        const run=status=>spawnSync('bash',[runner,out,'0,0 480x620'],{env:{...process.env,PATH:tmp+':'+process.env.PATH,DEMO_STATUS:JSON.stringify(status)},encoding:'utf8'});
        assert.notEqual(run({configured:true,demo:false,demoReady:false,loading:false}).status,0);
        assert.equal(fs.existsSync(out),false);
        assert.equal(run({configured:true,demo:true,demoReady:true,loading:false}).status,0);
        assert.equal(fs.readFileSync(out,'utf8'),'fixture image');
        assert.notEqual(run({configured:true,demo:true,demoReady:true,loading:false}).status,0);
        assert.equal(fs.readFileSync(out,'utf8'),'fixture image');
        assert.equal(fs.readdirSync(tmp).some(x=>x.includes('.tmp.')),false);
    } finally {fs.rmSync(tmp,{recursive:true,force:true});}
});
