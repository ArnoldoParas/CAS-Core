import '../App.css';
import sunIcon from '../assets/icons/sun.svg';
import chartIcon from '../assets/icons/chart.svg';
import pdfIcon from '../assets/icons/pdf.svg';

export default function Sidebar({ currentView, changeView, toggleTheme }) {
  return (
    <div className="sidebar">
      <div className='sidebar-header'>
        <h2>Menu</h2>
        <button onClick={toggleTheme}>
          <img src={sunIcon} alt="App Icon" className='icon' />
        </button>
      </div>
      <ul>
        <li>
          <button
            className={currentView === 'general' ? 'active' : ''}
            onClick={() => changeView('general')}>
            <img src={chartIcon} alt="App Icon" className='icon' />
            <p>Analisis</p>
          </button>
        </li>
        <li>
          <button
            className={currentView === 'pdf' ? 'active' : ''}
            onClick={() => changeView('pdf')}>
            <img src={pdfIcon} alt="App Icon" className='icon' />
            <p>PDF</p>
          </button>
        </li>
      </ul>
    </div>
  );
}

