htmx.onLoad(function() {
  document.body.addEventListener("todoFormReset", function(evt){
      document.getElementById('create-todo-form').reset();
  })
})
