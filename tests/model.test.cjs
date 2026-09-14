const {test}=require('node:test');const assert=require('node:assert/strict');const vm=require('node:vm');const fs=require('node:fs');
const M=vm.createContext({});vm.runInContext(fs.readFileSync(__dirname+'/../Model.js','utf8'),M);
const copy=x=>JSON.parse(JSON.stringify(x));const fixture=JSON.parse(fs.readFileSync(__dirname+'/../demo/fixtures/matches.json')).matches;
function game(sport){return copy(fixture.find(m=>m.sport===sport))}
function ingest(s,m,t=1000,emit=true){return M.ingest(s,[m],['cricket:england','nfl:chicago bears','football:arsenal','rugby:gloucester'],t,150000,emit)}
test('first load is silent for every sport',()=>{for(const m of fixture) assert.equal(ingest(M.tracker(),copy(m)).length,0)});
test('one wicket alerts once, with updated score',()=>{let s=M.tracker(),m=game('cricket');ingest(s,copy(m));m.innings[0].wickets++;let e=ingest(s,copy(m),2000);assert.equal(e.length,1);assert.equal(e[0].kind,'wicket');assert.match(e[0].body,/186\/5/);assert.equal(ingest(s,m,3000).length,0)});
test('runs alone are silent',()=>{let s=M.tracker(),m=game('cricket');ingest(s,copy(m));m.innings[0].runs+=4;assert.equal(ingest(s,m,2000).length,0)});
test('new innings is silent even if first seen after a wicket',()=>{let s=M.tracker(),m=game('cricket');ingest(s,copy(m));m.innings=[{id:'Australia Inning 1',runs:20,wickets:1,overs:'3'}];assert.equal(ingest(s,copy(m),2000).length,0);m.innings[0].wickets=2;assert.equal(ingest(s,m,3000)[0].kind,'wicket')});
test('score correction and replay do not repeat an alert',()=>{for(const sport of ['nfl','rugby','football']){let s=M.tracker(),m=game(sport);ingest(s,copy(m));m.teams[0].score++;assert.equal(ingest(s,copy(m),2000).length,1);m.teams[0].score--;assert.equal(ingest(s,copy(m),3000).length,0);m.teams[0].score++;assert.equal(ingest(s,m,4000).length,0)}});
test('wicket correction and replay are silent',()=>{let s=M.tracker(),m=game('cricket');ingest(s,copy(m));m.innings[0].wickets--;ingest(s,copy(m),2000);m.innings[0].wickets++;assert.equal(ingest(s,m,3000).length,0)});
test('several wickets are explicitly aggregated',()=>{let s=M.tracker(),m=game('cricket');ingest(s,copy(m));m.innings[0].wickets+=2;assert.match(ingest(s,m,2000)[0].title,/2 wickets since last update/)});
test('long outage re-baselines',()=>{let s=M.tracker(),m=game('cricket');ingest(s,copy(m));m.innings[0].wickets++;assert.equal(ingest(s,copy(m),999999).length,0);m.innings[0].wickets++;assert.equal(ingest(s,m,1000000).length,1)});
test('mute advances baseline without later catch-up',()=>{let s=M.tracker(),m=game('cricket');ingest(s,copy(m));m.innings[0].wickets++;assert.equal(ingest(s,copy(m),2000,false).length,0);assert.equal(ingest(s,m,3000,true).length,0)});
test('unfollowed teams and identically named teams in other sports stay silent',()=>{let s=M.tracker(),m=game('cricket');M.ingest(s,[copy(m)],['football:england'],1000,10000,true);m.innings[0].wickets++;assert.equal(M.ingest(s,[m],['football:england'],2000,10000,true).length,0)});
test('finish includes final score change and does not replay',()=>{let s=M.tracker(),m=game('nfl');ingest(s,copy(m));m.state='finished';m.teams[0].score+=3;let e=ingest(s,copy(m),2000);assert.equal(e.length,2);assert.equal(ingest(s,copy(m),3000).length,0);m.state='live';ingest(s,copy(m),4000);m.state='finished';assert.equal(ingest(s,m,5000).length,0)});
test('missing scores never become zero or trigger false goal',()=>{let s=M.tracker(),m=game('football');m.teams[0].score=null;ingest(s,copy(m));m.teams[0].score=2;assert.equal(ingest(s,m,2000).length,0)});
test('remote notification markup is stripped',()=>assert.equal(M.clean('<b>A&B</b>\n'), ' b A B /b  '));
test('explicit bar focus overrides order and keeps a final result pinned',()=>{const games=copy(fixture);const chosen=games[3];chosen.state='finished';assert.equal(M.focusedMatch(games,chosen.id).id,chosen.id);assert.equal(M.focusedMatch(games.reverse(),chosen.id).id,chosen.id)});
test('missing focused event never silently switches matches',()=>assert.equal(M.focusedMatch(fixture,'gone'),null));
test('cricket bar shows batting side, wickets and overs',()=>assert.equal(M.barScore(game('cricket'),false),'ENG 186/4 · 42.3 ov'));
test('football bar identifies both teams',()=>assert.equal(M.barScore(game('football'),false),'ARS 2 – LIV 1'));
