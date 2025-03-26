import '../App.css';

export default function Sidebar({ currentView, changeView, toggleTheme }) {
  

  return (
    <div className="sidebar">
      <div className='sidebar-header'>
        <h2>Menu</h2>
        <button onClick={toggleTheme}>
          <img src="../src/assets/icons/sun.svg" alt="App Icon" className='icon' />
        </button>
      </div>
      <ul>
        <li>
          <button
            className={currentView === 'general' ? 'active' : ''}
            onClick={() => changeView('general')}>
            <img src="../src/assets/icons/chart.svg" alt="App Icon" className='icon' />
            <p>Analisis</p>
          </button>
        </li>
        <li>
          <button
            className={currentView === 'pdf' ? 'active' : ''}
            onClick={() => changeView('pdf')}>
            <img src="../src/assets/icons/pdf.svg" alt="App Icon" className='icon' />
            <p>PDF</p>
          </button>
        </li>
        <li>
          <button
            className={currentView === 'etiquetas' ? 'active' : ''}
            onClick={() => changeView('etiquetas')}>
            <img src="../src/assets/icons/tag.svg" alt="App Icon" className='icon' />
            <p>Etiqueta</p>
          </button>
        </li>
      </ul>
    </div>
  )
}

