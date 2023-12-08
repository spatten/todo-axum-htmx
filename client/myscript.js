htmx.onLoad(function () {
  // reset the form after creating a new todo
  document.body.addEventListener("todoFormReset", function (evt) {
    document.getElementById('create-todo-form').reset();
  })

  // Sorting of todos
  var sortable = document.getElementById("todos");
  var sortableInstance = new Sortable(sortable, {
    animation: 150,
    ghostClass: 'blue-background-class',

    // Disable sorting on the `end` event
    onEnd: function (evt) {
      console.log(`onEnd triggered.`);
      this.option("disabled", true);
      htmx.trigger("#todos", "drop-end")
    }
  });

  // Re-enable sorting on the `htmx:afterSwap` event
  sortable.addEventListener("htmx:afterSwap", function () {
    sortableInstance.option("disabled", false);
  });
})
