htmx.onLoad(function () {
  // reset the form after creating a new todo
  document.body.addEventListener("todoFormReset", function (evt) {
    document.getElementById('create-todo-form').reset();
  })
  console.log("here I am");

  function setupSortable() {
    var sortable = document.getElementById("todos");
    var sortableInstance = new Sortable(sortable, {
      animation: 150,
      ghostClass: 'blue-background-class',

      // Disable sorting on the `end` event
      onEnd: function (evt) {
        console.log(`onEnd triggered.`);
        // this.option("disabled", true);
        htmx.trigger("#todos", "drop-end")
      }
    });

    return sortable;
  }
  sortable = setupSortable();

  sortable.addEventListener("htmx:afterSwap", function () {
    console.log("re-enabling sort")
    setupSortable();
  });
  // Sorting of todos

  // Re-enable sorting on the `htmx:afterSwap` event
})
