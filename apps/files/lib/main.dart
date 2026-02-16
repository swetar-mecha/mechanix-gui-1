import 'dart:io';

import 'package:dbus/dbus.dart';
import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:mechanix_files/app_config.dart';
import 'package:mechanix_files/app_route.dart';
import 'package:mechanix_files/load_settings.dart';
import 'package:mechanix_files/src/commons/customWidgets/fps_overlay.dart';
import 'package:mechanix_files/src/features/files/blocs/file_boc.dart';
import 'package:mechanix_files/src/features/files/blocs/file_event.dart';
import 'package:mechanix_files/src/features/files/data/file_repository.dart';
import 'package:mechanix_files/src/features/files/data/file_repository_impl.dart';
import 'package:mechanix_files/src/features/files/data/recent_file_manager_repository.dart';
import 'package:mechanix_files/src/features/files/presentation/files_home.dart';
import 'package:watch_it/watch_it.dart';
import 'package:widgets/mechanix.dart';

/// ---------------------------------------------------------------------------
/// APP ENTRY
/// ---------------------------------------------------------------------------

Future<void> main(List<String> args) async {
  WidgetsFlutterBinding.ensureInitialized();

  // Global, stable singletons
  di.registerSingleton(ThemeToggle());
  di.registerSingleton<_ThemeController>(_ThemeController());

  final configResult = await connectToMxconf();
  AppConfig().loadFromMap(configResult);

  final openPath = _parseOpenPath();

  runApp(
    MultiRepositoryProvider(
      providers: [
        RepositoryProvider<RecentFilesManager>(
          create: (_) => RecentFilesManager(),
        ),
        RepositoryProvider<FileRepository>(
          create: (_) => FileRepositoryImpl(),
        ),
      ],
      child: MechanixFilesApp(openPath: openPath),
    ),
  );
}

String _parseOpenPath() {
  const compileTimeOpenPath =
      String.fromEnvironment('MECHANIX_FILES_OPEN_PATH');
  final runtimeOpenPath = Platform.environment['MECHANIX_FILES_OPEN_PATH'];

  return compileTimeOpenPath.isNotEmpty
      ? compileTimeOpenPath
      : (runtimeOpenPath ?? '');
}

/// ---------------------------------------------------------------------------
/// ROOT APP (LISTENS ONLY TO THEME MODE)
/// ---------------------------------------------------------------------------

class MechanixFilesApp extends WatchingWidget {
  const MechanixFilesApp({
    super.key,
    required this.openPath,
  });

  final String openPath;

  @override
  Widget build(BuildContext context) {
    final themeMode = watchPropertyValue(
      (ThemeToggle t) => t.themeMode,
    );

    return _ThemeRoot(
      openPath: openPath,
      themeMode: themeMode,
    );
  }
}

/// ---------------------------------------------------------------------------
/// THEME ROOT (DBUS + MECHANIX THEME HANDLING)
/// ---------------------------------------------------------------------------

class _ThemeRoot extends StatefulWidget {
  const _ThemeRoot({
    required this.openPath,
    required this.themeMode,
  });

  final String openPath;
  final ThemeMode themeMode;

  @override
  State<_ThemeRoot> createState() => _ThemeRootState();
}

class _ThemeRootState extends State<_ThemeRoot> {
  final _ThemeController _controller = di<_ThemeController>();

  MechanixThemeData _themeData = const MechanixThemeData(
    mechanixVariant: MechanixVariant.amber,
  );

  @override
  void initState() {
    super.initState();
    _controller.init(_onThemeChanged);
  }

  void _onThemeChanged(MechanixThemeData data) {
    if (!mounted) return;
    setState(() => _themeData = data);
  }

  @override
  void dispose() {
    _controller.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    return MechanixTheme(
      data: _themeData,
      child: _MainApp(
        openPath: widget.openPath,
        themeMode: widget.themeMode,
      ),
    );
  }
}

/// ---------------------------------------------------------------------------
/// MAIN APP (BLOCS + MATERIAL APP)
/// ---------------------------------------------------------------------------

class _MainApp extends StatefulWidget {
  const _MainApp({
    required this.openPath,
    required this.themeMode,
  });

  final String openPath;
  final ThemeMode themeMode;

  @override
  State<_MainApp> createState() => _MainAppState();
}

class _MainAppState extends State<_MainApp> {
  late ThemeData _cachedDarkTheme;

  @override
  void didChangeDependencies() {
    super.didChangeDependencies();

    final mechanix = MechanixTheme.of(context);

    // Cache theme to avoid rebuild allocations
    _cachedDarkTheme = mechanix.darkTheme.copyWith(
      pageTransitionsTheme: const PageTransitionsTheme(
        builders: {
          TargetPlatform.linux: CupertinoPageTransitionsBuilder(),
        },
      ),
    );
  }

  @override
  Widget build(BuildContext context) {
    final mechanix = MechanixTheme.of(context);

    return MultiBlocProvider(
      providers: [
        BlocProvider<FilesBloc>(
          create: (_) => FilesBloc(
            fileRepository: context.read<FileRepository>(),
            recentFilesManager: context.read<RecentFilesManager>(),
          ),
        ),
      ],
      child: MaterialApp(
        debugShowCheckedModeBanner: false,
        theme: mechanix.lightTheme,
        darkTheme: _cachedDarkTheme,
        themeMode: widget.themeMode,
        home: FileHomePage(
          path: widget.openPath.isNotEmpty
              ? pathToSegments(widget.openPath)
              : const [],
        ),
        routes: {
          AppRoutes.files: (_) => const FileHomePage(),
        },
      ),
    );
  }
}

/// ---------------------------------------------------------------------------
/// THEME CONTROLLER (DBUS LIFECYCLE OUTSIDE UI LOGIC)
/// ---------------------------------------------------------------------------

class _ThemeController {
  late final DBusClient _bus;
  late final ThemeSettingsService _service;

  void init(void Function(MechanixThemeData) onChange) {
    _bus = DBusClient.session();
    _service = ThemeSettingsService(_bus);

    _service.listenForThemeChanges((colors) {
      final data = _service.colorsToThemeData(colors);
      onChange(data);
    });

    _loadInitialTheme(onChange);
  }

  Future<void> _loadInitialTheme(
    void Function(MechanixThemeData) onChange,
  ) async {
    final colors = await _service.fetchCurrentTheme();
    if (colors != null) {
      onChange(_service.colorsToThemeData(colors));
    }
  }

  void dispose() {
    _service.dispose();
    _bus.close();
  }
}

/// ---------------------------------------------------------------------------
/// FILES BLOC (FIXED: NO UI-SIDE EFFECTS)
/// ---------------------------------------------------------------------------

// class FilesBloc extends Bloc<FilesEvent, FilesState> {
//   FilesBloc({
//     required FileRepository fileRepository,
//     required RecentFilesManager recentFilesManager,
//   }) : super(const FilesState.initial()) {
//     on<InitializeFiles>(_onInitialize);

//     // ✅ Safe self-initialization
//     add(InitializeFiles());
//   }

//   Future<void> _onInitialize(
//     InitializeFiles event,
//     Emitter<FilesState> emit,
//   ) async {
//     // lazy / paginated file loading logic
//   }
// }
