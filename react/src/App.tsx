export default function App() {
  const handleIndex = async () => {
    const data = await fetch("https://localhost:8000/", {
      method: "GET",
      credentials: "include",
    });
    console.log(await data.text());
  };

  const handleLogin = async () => {
    const data = await fetch("https://localhost:8000/login", {
      method: "POST",
      credentials: "include",
      body: JSON.stringify({
        username: "user",
        password: "password",
      }),
      headers: { "Content-Type": "application/json" },
    }).then((res) => res.json());
    console.log(data);
  };

  const handleVlalid = async () => {
    const data = await fetch("https://localhost:8000/validate", {
      method: "GET",
      credentials: "include",
    }).then((res) => res.json());
    console.log(data);
  };

  return (
    <>
      <button onClick={handleIndex}>index</button>
      <button onClick={handleLogin}>login</button>
      <button onClick={handleVlalid}>valid</button>
    </>
  );
}
