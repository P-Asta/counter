document.getElementById("up")
    .addEventListener("click", () =>{
        fetch("http://localhost:8080/up", {method: "POST"})
        setTimeout(()=>{
            fetch("http://localhost:8080/get").then(e => { e.json().then( e => {
                if (e.status == 200){
                    document.getElementById("cnt").innerHTML = e.point
                }else{
                    document.getElementById("cnt").innerHTML = "ERROR"
                }
            })})
        }, 5)
    })

document.getElementById("down")
    .addEventListener("click", () =>{
        fetch("http://localhost:8080/down", {method: "POST"})
        setTimeout(()=>{
            fetch("http://localhost:8080/get").then(e => { e.json().then( e => {
                if (e.status == 200){
                    document.getElementById("cnt").innerHTML = e.point
                }else{
                    document.getElementById("cnt").innerHTML = "ERROR"
                }
            })})
        }, 5)
    })