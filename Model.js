function favourites(value) {
    return String(value || '').split(';').map(function(x) { return x.trim().toLowerCase() }).filter(function(x) { return x.length > 0 }).slice(0, 40)
}
function follows(match, selected) {
    return (match.teams || []).some(function(t) {
        return selected.indexOf(match.sport + ':' + String(t.name).toLowerCase()) >= 0
    })
}
function score(match) {
    if (match.sport === 'cricket') return (match.innings || []).map(function(i) { return i.id + ': ' + i.runs + '/' + (i.wickets === null ? '—' : i.wickets) + (i.overs ? ' (' + i.overs + ')' : '') }).join(' · ') || 'Yet to bat'
    return (match.teams || []).map(function(t) { return t.name + ' ' + (t.score === null ? '—' : t.score) }).join(' · ')
}
function tracker() { return { matches: {}, high: {} } }
function transition(old, match, high) {
    var events = []
    if (!old) return events
    if (old.state === 'scheduled' && match.state === 'live' && !high[match.id+':started']) events.push({kind:'start', title:'Match started'})
    if (old.state === 'live' && match.state === 'finished' && !high[match.id+':finished']) events.push({kind:'result', title:'Match finished'})
    if (old.state !== 'live' || (match.state !== 'live' && match.state !== 'finished')) return events
    if (match.sport === 'cricket') {
        ;(match.innings || []).forEach(function(i) {
            var previous = (old.innings || []).filter(function(x) { return x.id === i.id })[0]
            // A previously unseen innings is a baseline, never a wicket event.
            if (!previous || i.wickets === null || previous.wickets === null) return
            // RSS has score slots rather than authoritative innings IDs. Require
            // continuous observation of the same batting slot; resets are silent.
            if (match.source === 'espncricinfo-rss' && (old.activeInningsId !== i.id || match.activeInningsId !== i.id || i.runs < previous.runs || previous.declared)) return
            var key = match.id + ':' + i.id
            var ceiling = Math.max(previous.wickets, high[key] || 0)
            if (i.wickets > ceiling && i.wickets <= 10) events.push({kind:'wicket',title:(i.wickets-ceiling === 1 ? 'Wicket!' : (i.wickets-ceiling) + ' wickets since last update'),detail:i.id + ' · ' + i.runs + '/' + (i.wickets === null ? '—' : i.wickets) + (i.overs ? ' (' + i.overs + ')' : '')})
        })
    } else {
        ;(match.teams || []).forEach(function(t) {
            var previous = (old.teams || []).filter(function(x) { return x.id === t.id })[0]
            if (!previous || t.score === null || previous.score === null) return
            var key = match.id + ':' + t.id
            if (t.score > Math.max(previous.score, high[key] || 0)) events.push({kind:'score',title:t.name + ' score update'})
        })
    }
    return events
}
function ingest(state, matches, selected, now, maxGap, emit) {
    var events = []
    matches.forEach(function(match) {
        var previous = state.matches[match.id]
        if (emit && previous && now - previous.time <= maxGap && now >= previous.time && follows(match,selected)) {
            transition(previous.match, match, state.high).forEach(function(e) {
                e.body = match.name + '\n' + (e.detail || score(match))
                e.matchId = match.id
                events.push(e)
            })
        }
        state.matches[match.id] = {match:match,time:now}
        if (match.state === 'live' || match.state === 'finished') state.high[match.id+':started']=1
        if (match.state === 'finished') state.high[match.id+':finished']=1
        var values = match.sport === 'cricket' ? (match.innings || []) : (match.teams || [])
        values.forEach(function(v) {
            var n = match.sport === 'cricket' ? v.wickets : v.score
            var key = match.id + ':' + v.id
            if (n !== null) state.high[key] = Math.max(state.high[key] || 0,n)
        })
    })
    // Bound session state to 48 hours; restart intentionally re-baselines.
    Object.keys(state.matches).forEach(function(id) {
        if (now-state.matches[id].time>172800000) {
            delete state.matches[id]
            Object.keys(state.high).filter(function(k){return k.indexOf(id+':')===0}).forEach(function(k){delete state.high[k]})
        }
    })
    return events.slice(0,20)
}
function clean(value) { return String(value || '').replace(/[<>&\u0000-\u001f]/g, ' ').slice(0,500) }

// Focus is a display preference; it never narrows match notifications.
function focusedMatch(matches, id) {
    if (id) return matches.filter(function(m) { return m.id === id })[0] || null
    return matches.filter(function(m) { return m.state === 'live' })[0] || null
}
function shortTeam(name) { return String(name || '').trim().split(/\s+/)[0].slice(0,3).toUpperCase() }
function fixtureLabel(match) { return (match.teams || []).map(function(t) { return shortTeam(t.name) }).join(' v ') }
function barScore(match, vertical) {
    if (match.sport === 'cricket' && match.innings.length) {
        var i = match.innings.filter(function(x) { return x.id === match.activeInningsId })[0] || match.innings[match.innings.length-1]
        var batting = String(i.id).replace(/\s+Inning(?:s)?\s+\d+.*$/i, '')
        var figures = (i.runs === null ? '—' : i.runs) + '/' + (i.wickets === null ? '—' : i.wickets)
        return vertical ? figures.replace('/', '\n/') : shortTeam(batting) + ' ' + figures + (i.overs ? ' · ' + i.overs + ' ov' : '')
    }
    if (vertical) return match.teams.map(function(t) { return t.score === null ? '—' : t.score }).join('\n')
    return match.teams.map(function(t) { return shortTeam(t.name) + ' ' + (t.score === null ? '—' : t.score) }).join(' – ')
}
