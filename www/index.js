window.onload = function() {
    window.addEventListener('beforeunload', function(event) {
        event.preventDefault();
        event.returnValue = true;
    })
}