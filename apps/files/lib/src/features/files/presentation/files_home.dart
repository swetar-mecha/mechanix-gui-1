import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:mechanix_files/app_config.dart';
import 'package:mechanix_files/src/commons/constants.dart';
import 'package:mechanix_files/src/features/files/blocs/file_boc.dart';
import 'package:mechanix_files/src/features/files/blocs/file_event.dart';
import 'package:mechanix_files/src/features/files/models/types.dart';
import 'package:mechanix_files/src/features/files/presentation/files.dart';
import 'package:widgets/mechanix.dart';
import 'package:widgets/widgets/section_list/mechanix_section_list_theme.dart';
import 'package:widgets/widgets/section_list/section_list_items_type.dart';

class FileHomePage extends StatefulWidget {
  final String title;
  final List<FileItem> path;

  const FileHomePage({
    super.key,
    this.title = "Files",
    this.path = const [],
  });

  @override
  FileHomePageState createState() => FileHomePageState();
}

class FileHomePageState extends State<FileHomePage> {
  late final String downloadsDir;
  late final String documentsDir;
  late final String homeDir;
  late final String recentDir;

  @override
  void initState() {
    super.initState();

    final config = AppConfig();
    downloadsDir = config.downloadsDir;
    documentsDir = config.documentsDir;
    homeDir = config.homeDir;
    recentDir = config.recentDir;

    if (widget.path.isNotEmpty) {
      WidgetsBinding.instance.addPostFrameCallback((_) {
        if (!mounted) return;
        _navigateToPath(
          "/${widget.path.map((e) => e.name).join("/")}",
          widget.path,
        );
      });
    }
  }

  @override
  Widget build(BuildContext context) {
    final itemTitleStyle = _buildItemTitleStyle(context);
    final sectionTitleStyle = _buildSectionTitleStyle(context);
    final primaryColor = context.colorScheme.primary;
    final containerColor = context.colorScheme.primaryContainer;

    return Scaffold(
      appBar: AppBar(
        scrolledUnderElevation: 0,
        elevation: 0,
        backgroundColor: Colors.transparent,
        automaticallyImplyLeading: false,
        title: Text(
          "Files",
          style: TextStyle(
            fontSize: 32,
            color: primaryColor,
            fontWeight: FontWeight.w600,
          ),
        ),
      ),
      body: SingleChildScrollView(
        child: Container(
          margin: const EdgeInsets.all(12),
          child: Column(
            children: [
              MechanixSectionList(
                sectionListItems: [
                  SectionListItems.leadingIcon(
                    title: "Home directory",
                    titleTextStyle: itemTitleStyle,
                    onTap: () => _navigateToDirectory(homeDir, "Home"),
                    iconColor: containerColor,
                    iconPath: Images.home,
                  ),
                  SectionListItems.leadingIcon(
                    title: "Recents",
                    titleTextStyle: itemTitleStyle,
                    onTap: () => _navigateToRecents(),
                    iconColor: containerColor,
                    iconPath: Images.recent,
                  ),
                  SectionListItems.leadingIcon(
                    title: "Downloads",
                    titleTextStyle: itemTitleStyle,
                    onTap: () =>
                        _navigateToDirectory(downloadsDir, "Downloads"),
                    iconColor: containerColor,
                    iconPath: Images.downloads,
                  ),
                  SectionListItems.leadingIcon(
                    title: "Documents",
                    titleTextStyle: itemTitleStyle,
                    onTap: () =>
                        _navigateToDirectory(documentsDir, "Documents"),
                    iconColor: containerColor,
                    iconPath: Images.homeDocuments,
                  ),
                ],
              ),
              MechanixSectionList(
                title: 'Hard Drive',
                theme: MechanixSectionListThemeData(
                  titleTextStyle: sectionTitleStyle,
                ),
                sectionListItems: [
                  SectionListItems.leadingIcon(
                    title: "Root (/)",
                    titleTextStyle: itemTitleStyle,
                    onTap: () => _navigateToDirectory("/", "Root"),
                    iconColor: containerColor,
                    iconPath: Images.hardDrive,
                  ),
                ],
              ),
            ],
          ),
        ),
      ),
    );
  }

  TextStyle _buildItemTitleStyle(BuildContext context) {
    return TextStyle(
      fontSize: 20,
      color: context.colorScheme.onSurface,
      fontWeight: FontWeight.w500,
    );
  }

  TextStyle _buildSectionTitleStyle(BuildContext context) {
    return TextStyle(
      fontSize: 20,
      color: context.colorScheme.onSecondaryFixed,
      fontWeight: FontWeight.w500,
    );
  }

  void _navigateToRecents() async {
    if (!mounted) return;
    final filesBloc = context.read<FilesBloc>();

    // Pre-build the route before navigation
    await _navigateWithPrebuiltSlide(
      bloc: filesBloc,
      page: FileExplorerPage(
        title: 'Recent',
        path: pathToSegments(recentDir),
      ),
      onNavigated: () {
        if (mounted) filesBloc.add(LoadRecentFiles());
      },
    );
  }

  void _navigateToDirectory(String path, String title) async {
    if (!mounted) return;
    final filesBloc = context.read<FilesBloc>();

    await _navigateWithPrebuiltSlide(
      bloc: filesBloc,
      page: FileExplorerPage(startPath: path),
    );
  }

  void _navigateToPath(String startPath, List<FileItem> path) async {
    if (!mounted) return;
    final filesBloc = context.read<FilesBloc>();

    await _navigateWithPrebuiltSlide(
      bloc: filesBloc,
      page: FileExplorerPage(
        startPath: startPath,
        path: path,
      ),
    );
  }

  // CRITICAL FIX: Pre-build and warm up the route before navigating
  // Future<void> _navigateWithPrebuiltSlide({
  //   required FilesBloc bloc,
  //   required Widget page,
  //   VoidCallback? onNavigated,
  // }) async {
  //   // Give the UI thread a frame to prepare
  //   await Future.delayed(const Duration(milliseconds: 50));

  //   if (!mounted) return;

  //   final route = PageRouteBuilder<void>(
  //     pageBuilder: (context, animation, secondaryAnimation) {
  //       // Wrap in RepaintBoundary to isolate rendering
  //       return RepaintBoundary(
  //         child: BlocProvider.value(
  //           value: bloc,
  //           child: page,
  //         ),
  //       );
  //     },
  //     transitionsBuilder: (context, animation, secondaryAnimation, child) {
  //       const begin = Offset(1.0, 0.0);
  //       const end = Offset.zero;
  //       const curve = Curves.easeOutCubic;

  //       final tween =
  //           Tween(begin: begin, end: end).chain(CurveTween(curve: curve));

  //       // Wrap SlideTransition in RepaintBoundary
  //       return RepaintBoundary(
  //         child: SlideTransition(
  //           position: animation.drive(tween),
  //           child: child,
  //         ),
  //       );
  //     },
  //     transitionDuration: const Duration(milliseconds: 250),
  //   );

  //   // Navigate with the pre-built route
  //   await Navigator.push(context, route);
  //   onNavigated?.call();
  // }

  Future<void> _navigateWithPrebuiltSlide({
    required FilesBloc bloc,
    required Widget page,
    VoidCallback? onNavigated,
  }) async {
    if (!mounted) return;

    await Navigator.push<void>(
      context,
      MaterialPageRoute(
        builder: (context) => BlocProvider.value(
          value: bloc,
          child: page,
        ),
      ),
    );

    onNavigated?.call();
  }
}

List<FileItem> pathToSegments(String fullPath) {
  final segments =
      fullPath.split('/').where((segment) => segment.isNotEmpty).toList();

  return segments.map((name) {
    return FileItem(name: name, type: 'dir', children: null);
  }).toList();
}
